use std::{error::Error, fs::create_dir, path::Path};

use image::{open, DynamicImage, GenericImage, GenericImageView};

use crate::{check_if_image, image_writer};

pub trait Decompiler {
    fn decompile(&self, dir: &Path) -> Result<(), Box<dyn Error>>;
}

/*
 * Decompilers for the image type `image::Rgba8Image`
 * The following decompilers are small simple decompilers
 * Anyone can use them to decompile an image
*/

///Decompiles an image through squares of a given dimension
pub struct SquareDecompiler {
    dim: u32,
}
impl SquareDecompiler {
    pub fn new(dim: u32) -> SquareDecompiler {
        SquareDecompiler { dim }
    }
}
impl Decompiler for SquareDecompiler {
    fn decompile(&self, img: &Path) -> Result<(), Box<dyn Error>> {
        assert!(check_if_image(img));

        println!("now decompiling: {}", img.display());

        let img = open(img).unwrap();
        let mut size = img.width();
        let mut counter = 0;

        while size > 0 {
            let mut new_img = DynamicImage::new_rgba8(self.dim, self.dim);

            for x in 0..new_img.height() {
                for y in 0..new_img.width() {
                    let n = y + counter * self.dim;
                    let pixel = img.get_pixel(n, x);
                    new_img.put_pixel(y, x, pixel);
                }
            }
            if !Path::new("./decompiledImages").exists() {
                create_dir(Path::new("./decompiledImages"))?;
            }

            image_writer(new_img, format!("./decompiledImages/{}.png", counter))?;

            size -= self.dim;
            counter += 1;
        }

        Ok(())
    }
}

///Decompiles an image through rectangles of given dimensions
pub struct RectangleDecompiler {
    width: u32,
    height: u32,
}
impl RectangleDecompiler {
    pub fn new(width: u32, height: u32) -> RectangleDecompiler {
        RectangleDecompiler { width, height }
    }
}
impl Decompiler for RectangleDecompiler {
    fn decompile(&self, img: &Path) -> Result<(), Box<dyn Error>> {
        assert!(check_if_image(img));

        println!("now decompiling: {}", img.display());

        let img = open(img).unwrap();
        let mut size = img.width();
        let mut counter = 0;

        if img.height() >= self.height {
            while size > 0 {
                let mut new_img = DynamicImage::new_rgba8(self.width, self.height);

                for x in 0..new_img.height() {
                    for y in 0..new_img.width() {
                        let n = y + counter * self.width;
                        let pixel = img.get_pixel(n, x);
                        new_img.put_pixel(y, x, pixel);
                    }
                }
                if !Path::new("./decompiledImages").exists() {
                    create_dir(Path::new("./decompiledImages"))?;
                }

                image_writer(new_img, format!("./decompiledImages/{}.png", counter))?;

                size -= self.width;
                counter += 1;
                print!("{}", counter);
            }
        } else {
            eprintln!("Image is too small to decompile");
        }

        Ok(())
    }
}
