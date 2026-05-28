use ratatui::style::Color;

#[derive(Clone, Copy)]
pub enum ThemePreset {
    DeepSpace,
    Nord,
    HighContrast,
    SolarizedDark,
    SolarizedLight,
    GruvboxDark,
    GruvboxLight,
    Monokai,
    OneDark,
    Dracula,
    TokyoNight,
    Catppuccin,
    Cyberpunk,
}

pub struct Theme {
    pub name: String,
    pub border: Color,
    pub title: Color,
    pub text: Color,
    pub highlight: Color,
    pub graph_primary: Color,
    pub graph_secondary: Color,
    pub graph_tertiary: Color,
    pub graph_quaternary: Color,
    pub process_header: Color,
}

impl Theme {
    pub fn from_preset(preset: ThemePreset) -> Self {
        match preset {
            ThemePreset::DeepSpace => Self::dark(),
            ThemePreset::Nord => Self::nord(),
            ThemePreset::HighContrast => Self::high_contrast(),
            ThemePreset::SolarizedDark => Self::solarized_dark(),
            ThemePreset::SolarizedLight => Self::solarized_light(),
            ThemePreset::GruvboxDark => Self::gruvbox_dark(),
            ThemePreset::GruvboxLight => Self::gruvbox_light(),
            ThemePreset::Monokai => Self::monokai(),
            ThemePreset::OneDark => Self::one_dark(),
            ThemePreset::Dracula => Self::dracula(),
            ThemePreset::TokyoNight => Self::tokyo_night(),
            ThemePreset::Catppuccin => Self::catppuccin(),
            ThemePreset::Cyberpunk => Self::cyberpunk(),
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "Deep Space".to_string(),
            border: Color::DarkGray,
            title: Color::Cyan,
            text: Color::Gray,
            highlight: Color::Cyan,
            graph_primary: Color::Green,
            graph_secondary: Color::Yellow,
            graph_tertiary: Color::Red,
            graph_quaternary: Color::Magenta,
            process_header: Color::Yellow,
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord".to_string(),
            border: Color::Indexed(4),
            title: Color::Indexed(6),
            text: Color::Indexed(7),
            highlight: Color::Indexed(14),
            graph_primary: Color::Indexed(14),
            graph_secondary: Color::Indexed(13),
            graph_tertiary: Color::Indexed(12),
            graph_quaternary: Color::Indexed(15),
            process_header: Color::Indexed(6),
        }
    }

    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".to_string(),
            border: Color::White,
            title: Color::Yellow,
            text: Color::White,
            highlight: Color::White,
            graph_primary: Color::Green,
            graph_secondary: Color::Yellow,
            graph_tertiary: Color::Red,
            graph_quaternary: Color::Magenta,
            process_header: Color::Cyan,
        }
    }

    pub fn solarized_dark() -> Self {
        Self {
            name: "Solarized Dark".to_string(),
            border: Color::Indexed(10),
            title: Color::Indexed(14),
            text: Color::Indexed(12),
            highlight: Color::Indexed(9),
            graph_primary: Color::Indexed(2),
            graph_secondary: Color::Indexed(3),
            graph_tertiary: Color::Indexed(1),
            graph_quaternary: Color::Indexed(5),
            process_header: Color::Indexed(4),
        }
    }

    pub fn solarized_light() -> Self {
        Self {
            name: "Solarized Light".to_string(),
            border: Color::Indexed(7),
            title: Color::Indexed(4),
            text: Color::Indexed(11),
            highlight: Color::Indexed(1),
            graph_primary: Color::Indexed(2),
            graph_secondary: Color::Indexed(3),
            graph_tertiary: Color::Indexed(9),
            graph_quaternary: Color::Indexed(5),
            process_header: Color::Indexed(14),
        }
    }

    pub fn gruvbox_dark() -> Self {
        Self {
            name: "Gruvbox Dark".to_string(),
            border: Color::Indexed(243),
            title: Color::Indexed(208),
            text: Color::Indexed(223),
            highlight: Color::Indexed(214),
            graph_primary: Color::Indexed(142),
            graph_secondary: Color::Indexed(214),
            graph_tertiary: Color::Indexed(167),
            graph_quaternary: Color::Indexed(132),
            process_header: Color::Indexed(109),
        }
    }

    pub fn gruvbox_light() -> Self {
        Self {
            name: "Gruvbox Light".to_string(),
            border: Color::Indexed(243),
            title: Color::Indexed(130),
            text: Color::Indexed(239),
            highlight: Color::Indexed(166),
            graph_primary: Color::Indexed(106),
            graph_secondary: Color::Indexed(172),
            graph_tertiary: Color::Indexed(124),
            graph_quaternary: Color::Indexed(96),
            process_header: Color::Indexed(66),
        }
    }

    pub fn monokai() -> Self {
        Self {
            name: "Monokai".to_string(),
            border: Color::DarkGray,
            title: Color::Indexed(197),
            text: Color::White,
            highlight: Color::Indexed(81),
            graph_primary: Color::Indexed(148),
            graph_secondary: Color::Indexed(208),
            graph_tertiary: Color::Indexed(197),
            graph_quaternary: Color::Indexed(141),
            process_header: Color::Indexed(81),
        }
    }

    pub fn one_dark() -> Self {
        Self {
            name: "One Dark".to_string(),
            border: Color::DarkGray,
            title: Color::Indexed(170),
            text: Color::Indexed(145),
            highlight: Color::Indexed(75),
            graph_primary: Color::Indexed(114),
            graph_secondary: Color::Indexed(180),
            graph_tertiary: Color::Indexed(168),
            graph_quaternary: Color::Indexed(170),
            process_header: Color::Indexed(75),
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "Dracula".to_string(),
            border: Color::Indexed(61),
            title: Color::Indexed(212),
            text: Color::Indexed(231),
            highlight: Color::Indexed(117),
            graph_primary: Color::Indexed(84),
            graph_secondary: Color::Indexed(228),
            graph_tertiary: Color::Indexed(212),
            graph_quaternary: Color::Indexed(141),
            process_header: Color::Indexed(117),
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".to_string(),
            border: Color::Indexed(61),
            title: Color::Indexed(111),
            text: Color::Indexed(110),
            highlight: Color::Indexed(120),
            graph_primary: Color::Indexed(120),
            graph_secondary: Color::Indexed(111),
            graph_tertiary: Color::Indexed(196),
            graph_quaternary: Color::Indexed(170),
            process_header: Color::Indexed(111),
        }
    }

    pub fn catppuccin() -> Self {
        Self {
            name: "Catppuccin".to_string(),
            border: Color::Indexed(146),
            title: Color::Indexed(183),
            text: Color::Indexed(153),
            highlight: Color::Indexed(217),
            graph_primary: Color::Indexed(156),
            graph_secondary: Color::Indexed(222),
            graph_tertiary: Color::Indexed(210),
            graph_quaternary: Color::Indexed(183),
            process_header: Color::Indexed(217),
        }
    }

    pub fn cyberpunk() -> Self {
        Self {
            name: "Cyberpunk".to_string(),
            border: Color::Magenta,
            title: Color::Yellow,
            text: Color::Cyan,
            highlight: Color::Magenta,
            graph_primary: Color::Green,
            graph_secondary: Color::Yellow,
            graph_tertiary: Color::Red,
            graph_quaternary: Color::Cyan,
            process_header: Color::Yellow,
        }
    }
}
