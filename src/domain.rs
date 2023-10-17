use std::{error::Error, sync::Arc};

#[derive(Debug, Clone, Copy)]
struct Hours(u8);

impl Hours {
    pub fn new(hours: u8) -> Option<Self> {
        if hours > 23 {
            return None;
        }

        Some(Hours(hours))
    }
}

#[derive(Debug, Clone, Copy)]
struct Minutes(u8);

impl Minutes {
    pub fn new(minutes: u8) -> Option<Self> {
        if minutes > 59 {
            return None;
        }

        Some(Minutes(minutes))
    }
}

#[derive(Debug, Clone, Copy)]
struct Time {
    hours: Hours,
    minutes: Minutes,
}

#[derive(Debug, Clone, Copy)]
struct TimeRange {
    start: Time,
    end: Time,
}

#[derive(Debug, Clone)]
struct Image {
    range: TimeRange,
    file: Arc<str>,
}

#[derive(Debug, Clone)]
struct ConfigFile {
    /// save the logs in a file
    debug_mode: bool,

    images: Arc<[Image]>,
}

impl ConfigFile {
    /// read the configuration from ~/.config/gnome-paper.config.toml
    pub fn read_from_str(file_content: &str) -> Result<Self, Box<dyn Error>> {
        todo!()
    }
}
