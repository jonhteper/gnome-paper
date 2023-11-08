use std::{path::PathBuf, process::Command, thread, time};

use chrono::{Duration, Local};

use crate::{
    application::Application,
    errors::AppResult,
    image::{Image, ImageLocation, ImageStartTime, _Image},
};

/// Copy test images to tmp directory
fn copy_images_to_tmp() {
    let origin_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/images");
    let _ = Command::new("cp")
        .arg("-R")
        .arg(origin_path)
        .arg("/tmp")
        .output()
        .expect("Error to copy test images");
}

#[test]
fn simple_app_test() -> AppResult<()> {
    copy_images_to_tmp();

    let now = Local::now();
    let image1_start = now + Duration::minutes(1);
    let image2_start = now + Duration::minutes(2);
    let image1 = Image::from(_Image {
        start: ImageStartTime::from(image1_start.time()),
        location: ImageLocation::try_from("/tmp/images/test1.jpg".to_string())?,
    });

    let image2 = Image::from(_Image {
        start: ImageStartTime::from(image2_start.time()),
        location: ImageLocation::try_from("/tmp/images/test2.jpg".to_string())?,
    });

    let mut app = Application::default()
        .set_images(vec![image1, image2])?
        .check()?;

    app.execute()?;

    thread::sleep(time::Duration::from_secs(60));

    app.execute()?;

    thread::sleep(time::Duration::from_secs(60));

    app.execute()?;

    Ok(())
}
