use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Color},
    symbols,
    text::Span,
    widgets::{Block, Borders, Paragraph, Row, Table, Cell, Chart, Dataset, Axis, GraphType, Tabs, Clear, List, ListItem},
    Frame,
};
use crate::app::App;
use crate::theme::Theme;

pub fn draw(f: &mut Frame, app: &App) {
    let theme = &app.theme;
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),      // Header
            Constraint::Percentage(55), // Top: Stats & Charts
            Constraint::Percentage(44), // Bottom: Processes
        ])
        .split(f.area());

    // Header
    let header_text = format!(" danmon | 'q': quit | 'g'/'Tab': next GPU | 't': themes | Theme: {} ", theme.name);
    let header = Paragraph::new(Span::styled(
        header_text,
        Style::default().fg(Color::Black).bg(theme.title).add_modifier(Modifier::BOLD),
    ));
    f.render_widget(header, chunks[0]);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Left: System & Net
            Constraint::Percentage(60), // Right: GPU Dashboard
        ])
        .split(chunks[1]);

    draw_left_panel(f, app, top_chunks[0]);
    draw_gpu_dashboard(f, app, top_chunks[1]);
    draw_process_table(f, app, chunks[2]);

    if app.show_theme_menu {
        draw_theme_menu(f, app);
    }
}

fn draw_left_panel(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // System Info
            Constraint::Length(6),  // Disk Table
            Constraint::Percentage(30), // CPU Chart
            Constraint::Percentage(30), // Net Chart
        ])
        .split(area);

    let (l1, l5, l15) = app.system_stats.load_avg();
    let (used_mem, total_mem) = app.system_stats.mem_usage();
    let mem_percent = if total_mem == 0 { 0 } else { (used_mem * 100 / total_mem) as u16 };
    let freqs = app.system_stats.cpu_frequencies();
    let avg_freq = if freqs.is_empty() { 0 } else { freqs.iter().sum::<u64>() / freqs.len() as u64 };
    let rx_latest = app.net_rx_history.back().cloned().unwrap_or(0.0);
    let tx_latest = app.net_tx_history.back().cloned().unwrap_or(0.0);
    
    let sys_info = format!(
        "Load: {:.2}, {:.2}, {:.2}\nCPU: {} MHz | Mem: {:.1}/{:.1} GB ({}%)\nNet RX: {:.1} KB/s | TX: {:.1} KB/s",
        l1, l5, l15, avg_freq,
        used_mem as f64 / 1e9, total_mem as f64 / 1e9,
        mem_percent,
        rx_latest / 1024.0, tx_latest / 1024.0
    );
    let p = Paragraph::new(sys_info)
        .style(Style::default().fg(theme.text))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)).title(Span::styled("System Overview", Style::default().fg(theme.title))));
    f.render_widget(p, chunks[0]);

    let disks = app.system_stats.disk_stats();
    let disk_rows: Vec<Row> = disks.iter().map(|(mount, used, total)| {
        let percent = if *total == 0 { 0.0 } else { (*used as f64 / *total as f64) * 100.0 };
        Row::new(vec![Cell::from(mount.clone()), Cell::from(format!("{:.1}%", percent))]).style(Style::default().fg(theme.text))
    }).collect();
    let disk_table = Table::new(disk_rows, [Constraint::Percentage(70), Constraint::Percentage(30)])
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)).title(Span::styled("Disks", Style::default().fg(theme.title))))
        .header(Row::new(vec!["Mount", "Used"]).style(Style::default().fg(theme.process_header)));
    f.render_widget(disk_table, chunks[1]);

    let cpu_data: Vec<(f64, f64)> = app.cpu_history.iter().enumerate().map(|(i, &v)| (i as f64, v as f64)).collect();
    let cpu_chart = Chart::new(vec![Dataset::default().name("CPU %").marker(symbols::Marker::Braille).graph_type(GraphType::Line).style(Style::default().fg(theme.graph_primary)).data(&cpu_data)])
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)).title(Span::styled("CPU History", Style::default().fg(theme.title))))
        .x_axis(Axis::default().bounds([0.0, 100.0]).style(Style::default().fg(theme.text)))
        .y_axis(Axis::default().bounds([0.0, 100.0]).labels(vec![Span::raw("0"), Span::raw("100")]).style(Style::default().fg(theme.text)));
    f.render_widget(cpu_chart, chunks[2]);

    let rx_data: Vec<(f64, f64)> = app.net_rx_history.iter().enumerate().map(|(i, &v)| (i as f64, (v as f64 / 1024.0).log10().max(0.0))).collect();
    let tx_data: Vec<(f64, f64)> = app.net_tx_history.iter().enumerate().map(|(i, &v)| (i as f64, (v as f64 / 1024.0).log10().max(0.0))).collect();
    let net_chart = Chart::new(vec![
        Dataset::default().name("RX").marker(symbols::Marker::Braille).graph_type(GraphType::Line).style(Style::default().fg(theme.graph_quaternary)).data(&rx_data),
        Dataset::default().name("TX").marker(symbols::Marker::Braille).graph_type(GraphType::Line).style(Style::default().fg(theme.graph_secondary)).data(&tx_data),
    ])
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)).title(Span::styled("Network (Log Scale)", Style::default().fg(theme.title))))
    .x_axis(Axis::default().bounds([0.0, 100.0]).style(Style::default().fg(theme.text)))
    .y_axis(Axis::default().bounds([0.0, 6.0]).labels(vec![Span::raw("1K"), Span::raw("1G")]).style(Style::default().fg(theme.text)));
    f.render_widget(net_chart, chunks[3]);
}

fn draw_gpu_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let gpus = app.gpu_stats.get_gpus();
    if gpus.is_empty() {
        let p = Paragraph::new("No NVIDIA GPU detected").style(Style::default().fg(theme.text)).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)).title(Span::styled("GPU Dashboard", Style::default().fg(theme.title))));
        f.render_widget(p, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(6), Constraint::Min(10)])
        .split(area);

    let titles: Vec<Span> = gpus.iter().enumerate().map(|(i, _)| {
        if i == app.selected_gpu { Span::styled(format!(" GPU {} ", i), Style::default().fg(theme.highlight).add_modifier(Modifier::BOLD)) }
        else { Span::raw(format!(" GPU {} ", i)) }
    }).collect();
    let tabs = Tabs::new(tabs_titles(gpus.len(), app.selected_gpu, theme))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)).title(Span::styled("Select GPU", Style::default().fg(theme.title))))
        .select(app.selected_gpu)
        .highlight_style(Style::default().fg(theme.highlight));
    f.render_widget(tabs, chunks[0]);

    let gpu = &gpus[app.selected_gpu];
    let mem_percent = if gpu.mem_total == 0 { 0 } else { (gpu.mem_used * 100 / gpu.mem_total) as u16 };
    let unified_gb = gpu.unified_mem_used as f64 / 1e9;
    
    let text = format!(
        "Model: {}\nClocks: G: {} MHz | M: {} MHz\nTemp: {}°C | Fan: {}% | Power: {:.1}W / {:.1}W\nLoad: {}% | Memory: {:.2} / {:.2} GB ({}%) | Unified: {:.2} GB",
        gpu.name, gpu.graphics_clock, gpu.memory_clock, gpu.temp, gpu.fan_speed,
        gpu.power_usage as f64 / 1000.0, gpu.power_limit as f64 / 1000.0,
        gpu.load, gpu.mem_used as f64 / 1e9, gpu.mem_total as f64 / 1e9, mem_percent, unified_gb
    );
    f.render_widget(Paragraph::new(text).style(Style::default().fg(theme.text)).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)).title(Span::styled(format!("GPU {} Status", app.selected_gpu), Style::default().fg(theme.title)))), chunks[1]);

    let load_data: Vec<(f64, f64)> = app.gpu_load_history.iter().enumerate().map(|(i, &v)| (i as f64, v as f64)).collect();
    let mem_data: Vec<(f64, f64)> = app.gpu_mem_history.iter().enumerate().map(|(i, &v)| (i as f64, v as f64)).collect();
    let power_data: Vec<(f64, f64)> = app.gpu_power_history.iter().enumerate().map(|(i, &v)| (i as f64, v as f64)).collect();
    let temp_data: Vec<(f64, f64)> = app.gpu_temp_history.iter().enumerate().map(|(i, &v)| (i as f64, v as f64)).collect();

    let chart = Chart::new(vec![
        Dataset::default().name("Load").marker(symbols::Marker::Braille).graph_type(GraphType::Line).style(Style::default().fg(theme.graph_primary)).data(&load_data),
        Dataset::default().name("Mem").marker(symbols::Marker::Braille).graph_type(GraphType::Line).style(Style::default().fg(theme.graph_secondary)).data(&mem_data),
        Dataset::default().name("Power").marker(symbols::Marker::Braille).graph_type(GraphType::Line).style(Style::default().fg(theme.graph_tertiary)).data(&power_data),
        Dataset::default().name("Temp").marker(symbols::Marker::Braille).graph_type(GraphType::Line).style(Style::default().fg(theme.graph_quaternary)).data(&temp_data),
    ])
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)).title(Span::styled("GPU Performance & Thermals", Style::default().fg(theme.title))))
    .x_axis(Axis::default().bounds([0.0, 100.0]).style(Style::default().fg(theme.text)))
    .y_axis(Axis::default().bounds([0.0, 100.0]).labels(vec![Span::raw("0"), Span::raw("100")]).style(Style::default().fg(theme.text)))
    .legend_position(Some(ratatui::widgets::LegendPosition::TopLeft));
    f.render_widget(chart, chunks[2]);
}

fn tabs_titles<'a>(count: usize, selected: usize, theme: &'a Theme) -> Vec<Span<'a>> {
    (0..count).map(|i| {
        if i == selected { Span::styled(format!(" GPU {} ", i), Style::default().fg(theme.highlight).add_modifier(Modifier::BOLD)) }
        else { Span::raw(format!(" GPU {} ", i)) }
    }).collect()
}

fn draw_process_table(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let processes = app.system_stats.processes();
    let gpus = app.gpu_stats.get_gpus();
    let mut gpu_proc_map: std::collections::HashMap<u32, (u64, Vec<usize>)> = std::collections::HashMap::new();
    for (i, gpu) in gpus.iter().enumerate() {
        for p in &gpu.processes {
            let entry = gpu_proc_map.entry(p.pid).or_insert((0, Vec::new()));
            entry.0 += p.used_mem; entry.1.push(i);
        }
    }

    let mut procs = processes.clone();
    procs.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let final_rows: Vec<Row> = procs.iter().take(area.height as usize).map(|(pid, name, cpu, mem)| {
        let (gpu_mem, gpu_ids) = gpu_proc_map.get(pid).cloned().unwrap_or((0, Vec::new()));
        Row::new(vec![
            Cell::from(pid.to_string()), Cell::from(name.clone()), Cell::from(format!("{:.1}", cpu)),
            Cell::from(format!("{:.1} MB", *mem as f64 / 1e6)),
            Cell::from(if gpu_mem > 0 { format!("{:.1} MB", gpu_mem as f64 / 1e6) } else { "-".into() }),
            Cell::from(if gpu_ids.is_empty() { "-".into() } else { gpu_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",") }),
        ]).style(Style::default().fg(theme.text))
    }).collect();

    let table = Table::new(final_rows, [Constraint::Length(8), Constraint::Min(15), Constraint::Length(8), Constraint::Length(12), Constraint::Length(12), Constraint::Length(8)])
        .header(Row::new(vec!["PID", "Name", "CPU %", "Mem", "GPU Mem", "GPUs"]).style(Style::default().fg(theme.process_header).add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)).title(Span::styled("Processes (Sorted by CPU)", Style::default().fg(theme.title))));
    f.render_widget(table, area);
}

fn draw_theme_menu(f: &mut Frame, app: &App) {
    let area = centered_rect(30, 50, f.area());
    let theme = &app.theme;
    
    let items: Vec<ListItem> = app.themes.iter().enumerate().map(|(i, t)| {
        let preset_theme = Theme::from_preset(*t);
        let style = if i == app.theme_index {
            Style::default().fg(theme.highlight).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text)
        };
        ListItem::new(preset_theme.name).style(style)
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Select Theme (Up/Down, Enter) ").border_style(Style::default().fg(theme.border)))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">> ");

    f.render_widget(Clear, area);
    f.render_widget(list, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
