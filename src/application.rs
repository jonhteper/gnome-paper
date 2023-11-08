use std::marker::PhantomData;

use chrono::{Local, NaiveTime};

use crate::{
    config::{Config, MAX_IMAGES_NUMBER},
    errors::{AppResult, Error},
    image::Image,
};

#[derive(Debug)]
pub struct Empty;

#[derive(Debug)]
pub struct Loaded;

#[derive(Debug)]
pub struct Executable;

#[derive(Debug)]
pub struct Application<State = Empty> {
    config: Config,

    /// Maximum 1440 = 1 for minute
    images: Vec<Image>,

    last_index_used: Option<usize>,

    state: PhantomData<State>,
}

impl Application {
    pub fn default() -> Application<Empty> {
        Application {
            config: Config::default(),
            images: Vec::with_capacity(MAX_IMAGES_NUMBER),
            last_index_used: None,
            state: PhantomData,
        }
    }

    #[inline]
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    #[inline]
    pub fn set_images(mut self, images: Vec<Image>) -> AppResult<Application<Loaded>> {
        if images.is_empty() {
            Err(Error::TooImagesLoaded)?
        }

        if images.len() == MAX_IMAGES_NUMBER {
            Err(Error::TooImagesLoaded)?
        }

        self.images = images;

        Ok(Application {
            config: self.config,
            images: self.images,
            last_index_used: None,
            state: PhantomData,
        })
    }
}

impl Application<Loaded> {
    /// Prevents an empty images list and sort it
    pub fn check(mut self) -> AppResult<Application<Executable>> {
        if self.images.is_empty() {
            Err(Error::NoImagesLoaded)?
        }

        self.images.sort();

        Ok(Application {
            config: self.config,
            images: self.images,
            last_index_used: None,
            state: PhantomData,
        })
    }
}

impl Application<Executable> {
    #[inline]
    fn execute_with_last_image(&self, last_index: usize, now: NaiveTime) -> AppResult<()> {
        if self.images.len() == 1 {
            return Ok(());
        }

        if let Some(next_image) = self.images.get(last_index + 1) {
            if next_image.start_time() <= now {
                next_image.set_to_wallpaper()?;
            }

            return Ok(());
        }

        let first_image = self.images.first().unwrap();
        let last_image = &self.images[last_index];

        if last_image.start_time() > now && first_image.start_time() <= now {
            first_image.set_to_wallpaper()?;
        }

        Ok(())
    }

    #[inline]
    fn execute_without_last_image(&mut self, now: NaiveTime) -> AppResult<()> {
        let mut last_coincident = None;
        let mut last_index = None;
        for (index, image) in self.images.iter().enumerate() {
            if image.start_time() <= now {
                last_coincident = Some(image);
                last_index = Some(index);
            }
        }

        if let Some(image) = last_coincident {
            image.set_to_wallpaper()?;
            self.last_index_used = last_index;
            return Ok(());
        }

        // if any coincident, the current valid image is the last.
        self.images.last().unwrap().set_to_wallpaper()?;
        self.last_index_used = Some(0);

        Ok(())
    }

    /// Search for the next image to set to wallpaper.
    /// Constraints:
    /// * The images vector isn't empty
    /// * The images vector is sorted
    pub fn execute(&mut self) -> AppResult<()> {
        let now = Local::now().time();
        if let Some(last_index) = self.last_index_used {
            self.execute_with_last_image(last_index, now)?;
        }

        self.execute_without_last_image(now)
    }
}
