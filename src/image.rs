use std::{env, ops::Deref, path::PathBuf, process::Command};

use chrono::{NaiveTime, ParseError};

use crate::errors::{AppResult, Error};

use serde_derive::{Deserialize, Serialize};

#[cfg(test)]
use named_ctor::NamedCtor;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ImageStartTime(NaiveTime);

impl ImageStartTime {
    pub fn as_naive_time(&self) -> NaiveTime {
        self.0
    }
}

impl From<NaiveTime> for ImageStartTime {
    fn from(value: NaiveTime) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for ImageStartTime {
    type Error = ParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let time = NaiveTime::parse_from_str(&s, "%H:%M")?;

        Ok(Self(time))
    }
}

impl From<ImageStartTime> for String {
    fn from(time: ImageStartTime) -> Self {
        time.0.format("%H:%M").to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct ImageLocation(PathBuf);

impl TryFrom<String> for ImageLocation {
    type Error = Error;

    fn try_from(file_path: String) -> Result<Self, Self::Error> {
        let file_location = match file_path.starts_with("~/") {
            true => {
                let home_var = env::var_os("HOME").ok_or(Error::NoHomeVar)?;
                let mut home_path = PathBuf::from(home_var);
                let file_path = file_path.strip_prefix("~/").unwrap();

                home_path.push(&file_path);

                home_path
            }
            false => PathBuf::from(&file_path),
        };

        let location = file_location
            .canonicalize()
            .map_err(|_| Error::InvalidLocation(file_path.clone()))?;

        Ok(Self(location))
    }
}

impl From<ImageLocation> for String {
    fn from(location: ImageLocation) -> Self {
        location.0.to_string_lossy().into()
    }
}

impl Deref for ImageLocation {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(NamedCtor))]
pub struct Image {
    start: ImageStartTime,
    location: ImageLocation,
}

impl Image {
    pub fn new(start: ImageStartTime, location: ImageLocation) -> Self {
        Self { start, location }
    }

    pub fn set_to_wallpaper(&self) -> AppResult<()> {
        let _ = Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri-dark") // TODO :set only if dark mode enabled
            .arg(&self.location.as_os_str())
            .output()
            .map_err(|err| Error::CommandError(err.to_string()))?;

        Ok(())
    }

    #[inline]
    pub fn start(&self) -> ImageStartTime {
        self.start
    }

    #[inline]
    pub fn start_time(&self) -> NaiveTime {
        self.start.as_naive_time()
    }

    #[inline]
    pub fn location(&self) -> &ImageLocation {
        &self.location
    }
}

impl PartialOrd for Image {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

impl Ord for Image {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::{Image, ImageLocation, ImageStartTime};

    #[test]
    fn image_test() {
        let start_time =
            ImageStartTime::try_from("20:00".to_string()).expect("error parsing image start time");
        let location = ImageLocation::try_from("/home/jonhteper/Imágenes/mizo.jpeg".to_string())
            .expect("error parsing image location");
        let image = Image::new(start_time, location);

        image
            .set_to_wallpaper()
            .expect("Error setting image to wallpaper");

        sleep(Duration::from_secs(3));
        let start_time =
            ImageStartTime::try_from("20:00".to_string()).expect("error parsing image start time");
        let location =
            ImageLocation::try_from("/home/jonhteper/Imágenes/breath-north-dark.png".to_string())
                .expect("error parsing image location");
        let image = Image::new(start_time, location);

        image
            .set_to_wallpaper()
            .expect("Error setting image to wallpaper");
    }
}
