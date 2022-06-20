use image::io::Reader;
use image::{open, DynamicImage, GenericImage};
use regex::Regex;
use std::error::Error;
use std::fs::{self, create_dir};
use std::path::Path;

use crate::{check_if_image, image_writer};

pub fn compile(dir: &Path, filter: &Option<Vec<String>>) -> Result<(), Box<dyn Error>> {
    assert!(dir.is_dir());
    if let Some(filt) = filter {
        if filt.contains(&dir.file_name().unwrap().to_str().unwrap().to_string()) {
            return Ok(());
        }
    }

    println!("now compiling: {}", dir.display());

    let dim = get_img_dim(dir);
    if let (0, 0) = dim {
        //Check if there are other directories to open
        let paths = fs::read_dir(dir).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                compile(&path, filter)?;
            }
        }
    } else {
        //Compile images
        let mut img = DynamicImage::new_rgba8(dim.0, dim.1);
        //Compile images from directory
        img = compile_dir(dir, img, filter)?;
        // //Write image
        if !Path::new("./compiledImages").exists() {
            create_dir(Path::new("./compiledImages"))?;
        }

        let paths = fs::read_dir("./compiledImages").unwrap();
        let mut counter = 0;
        for path in paths {
            let path = path.unwrap().path();
            let path = path.file_stem().unwrap().to_str().unwrap().to_string();
            let pattern = Regex::new(
                format!("^{}\\d?$", dir.file_name().unwrap().to_str().unwrap()).as_str(),
            )
            .unwrap();

            if pattern.is_match(&path) {
                counter += 1;
            }
        }
        write(counter, img, dir)?;
    }

    Ok(())
}

pub fn get_img_dim(dir: &Path) -> (u32, u32) {
    assert!(dir.is_dir());
    let mut dim = (0, 0);

    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if check_if_image(&path) {
            let img = open(path).unwrap();
            if img.height() > dim.1 {
                dim.1 = img.height();
            }
            dim.0 += img.width();
        }
    }

    dim
}

fn compile_dir(
    dir: &Path,
    mut img: DynamicImage,
    filter: &Option<Vec<String>>,
) -> Result<DynamicImage, Box<dyn Error>> {
    assert!(dir.is_dir());
    let mut x = 0;
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if check_if_image(&path) {
            let read = Reader::open(path)?.decode()?;
            img.copy_from(&read, x, 0)?;
            x += read.width();
        } else if path.is_dir() {
            compile(&path, filter)?;
        }
    }

    Ok(img)
}

fn write(counter: i32, img: DynamicImage, dir: &Path) -> Result<(), Box<dyn Error>> {
    if counter == 0 {
        image_writer(
            img,
            format!(
                "./compiledImages/{}.png",
                dir.file_name().unwrap().to_str().unwrap().to_owned()
            ),
        )?;
    } else {
        image_writer(
            img,
            format!(
                "./compiledImages/{}{}.png",
                dir.file_name().unwrap().to_str().unwrap().to_owned(),
                counter
            ),
        )?;
    }
    Ok(())
}
