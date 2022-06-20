use std::{
    error::Error,
    fs::{self},
    path::Path,
};

use image::{open, DynamicImage, GenericImage, GenericImageView};
use regex::Regex;

use crate::{commons::*, get_img_dim, Decompiler};

///Decompiles an image through rectangles of given dimensions
pub struct DirDecompiler {}
impl DirDecompiler {
    pub fn new() -> DirDecompiler {
        DirDecompiler {}
    }
}
impl Decompiler for DirDecompiler {
    fn decompile(&self, dir: &Path) -> Result<(), Box<dyn Error>> {
        assert!(dir.is_dir());

        println!("now decompiling based on: {}", dir.display());

        let base = fs::read_dir(dir).unwrap();
        let mut counter = 0;
        let mut compimage = DynamicImage::default();

        let compiled = fs::read_dir("./compiledImages/").unwrap();
        for comp in compiled {
            let comp = comp.unwrap().path();
            if dir.file_name() == comp.file_stem() {
                compimage = check_more_than_one(dir)?;
            }
        }

        for file in base {
            let file = file.unwrap().path();
            if check_if_image(file.as_path()) {
                let mut img = open(&file).unwrap();
                if compimage.dimensions() != (0, 0) {
                    for x in 0..img.height() {
                        for y in 0..img.width() {
                            let n = y + counter;
                            let pixel = compimage.get_pixel(n, x);
                            img.put_pixel(y, x, pixel);
                        }
                    }

                    counter += img.width();
                    image_writer(img, format!("{}", file.display()))?;
                }
            } else if file.is_dir() {
                self.decompile(&file)?;
            }
        }

        Ok(())
    }
}

fn check_more_than_one(dir: &Path) -> Result<DynamicImage, Box<dyn Error>> {
    let paths = fs::read_dir("./compiledImages").unwrap();
    let mut res = DynamicImage::default();

    for path in paths {
        let pathbuf = path.unwrap().path();
        let path = pathbuf.file_stem().unwrap().to_str().unwrap().to_string();
        let pattern =
            Regex::new(format!("^{}\\d?$", dir.file_name().unwrap().to_str().unwrap()).as_str())
                .unwrap();

        if pattern.is_match(&path) || path == dir.file_name().unwrap().to_str().unwrap() {
            if open(&pathbuf).unwrap().dimensions() == get_img_dim(dir) {
                res = open(pathbuf).unwrap();
            }
        }
    }
    Ok(res)
}
