use std::{env, path::PathBuf, time::Duration};

#[cfg(test)]
use named_ctor::NamedCtor;

use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub const MAX_IMAGES_NUMBER: usize = 1440;

static MINUTE: Duration = Duration::from_secs(60);
static TEN_MINUTES: Duration = Duration::from_secs(60 * 10);
static HALF_HOUR: Duration = Duration::from_secs(60 * 30);
static HOUR: Duration = Duration::from_secs(60 * 60);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr, Default)]
#[repr(u8)]
pub enum DebugMode {
    /// Errors not saved
    Off = 0,

    /// Errors saved in `/tmp/gpaper.log`
    #[default]
    TempFile = 1,

    /// Errors saved in `~/.config/gpaper/gpaper.log`
    File = 2,
}

impl DebugMode {
    pub fn location(&self) -> Option<PathBuf> {
        match self {
            DebugMode::Off => None,
            DebugMode::TempFile => Some(PathBuf::from("/tmp/gpaper.log")),
            DebugMode::File => {
                let home = env::var_os("HOME").unwrap();
                let path = PathBuf::from(home).join(".config/gpaper/gpaper.log");

                Some(path)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr, Default)]
#[repr(u8)]
pub enum Accuracy {
    /// Every minute is reloaded.
    #[default]
    Standard = 0,

    /// Every 10 minutes is reloaded.
    Lazy = 1,

    /// Every half hour is reloaded.
    HalfHourly = 2,

    /// Every hour is reloaded.
    Hourly = 3,
}

impl Accuracy {
    pub fn duration(&self) -> Duration {
        match self {
            Accuracy::Standard => MINUTE,
            Accuracy::Lazy => TEN_MINUTES,
            Accuracy::HalfHourly => HALF_HOUR,
            Accuracy::Hourly => HOUR,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(NamedCtor))]
pub struct Config {
    /// Errors storage
    debug_mode: DebugMode,

    /// Daemon reload interval
    accuracy: Accuracy,

    /// Indicates if the daemon run at start
    is_active: bool,
}

impl Config {
    pub fn default() -> Config {
        Config {
            debug_mode: DebugMode::default(),
            accuracy: Accuracy::default(),
            is_active: true,
        }
    }
}
