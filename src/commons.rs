use std::{error::Error, ffi::OsStr, path::Path};

use image::DynamicImage;

pub fn check_if_image(path: &Path) -> bool {
    if path.is_file() {
        let ext = path.extension().and_then(OsStr::to_str).unwrap();
        match ext {
            "png" | "bmp" | "jpeg" | "jpg" => true,
            _ => false,
        }
    } else {
        false
    }
}

pub fn image_writer(img: DynamicImage, name: String) -> Result<(), Box<dyn Error>> {
    img.save(name)?;

    Ok(())
}
