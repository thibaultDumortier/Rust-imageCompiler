use image::io::Reader;
use image::{open, DynamicImage, GenericImage};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, create_dir};
use std::path::Path;

pub fn compile(dir: &Path) -> Result<(), Box<dyn Error>> {
    assert!(dir.is_dir());

    println!("now compiling: {}", dir.display());

    let dim = get_img_dim(dir);
    if let (0, 0) = dim {
        //Check if there are other directories to open
        let paths = fs::read_dir(dir).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                compile(&path)?;
            }
        }
    } else {
        //Compile images
        let mut img = DynamicImage::new_rgba8(dim.0, dim.1);
        //Compile images from directory
        img = compile_dir(dir, img)?;
        // //Write image
        image_writer(img, dir.file_name().unwrap().to_str().unwrap().to_owned())?;
    }

    Ok(())
}

fn get_img_dim(dir: &Path) -> (u32, u32) {
    assert!(dir.is_dir());
    let mut dim = (0, 0);

    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let ext = path.extension().and_then(OsStr::to_str).unwrap();
            match ext {
                "png" | "bmp" | "jpeg" | "jpg" => {
                    let img = open(path).unwrap();
                    if img.height() > dim.1 {
                        dim.1 = img.height();
                    }
                    dim.0 += img.width();
                }
                _ => (),
            }
        }
    }

    dim
}

fn compile_dir(dir: &Path, mut img: DynamicImage) -> Result<DynamicImage, Box<dyn Error>> {
    assert!(dir.is_dir());
    let mut x = 0;
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let ext = path.extension().and_then(OsStr::to_str).unwrap();
            match ext {
                "png" | "bmp" | "jpeg" | "jpg" => {
                    let read = Reader::open(path)?.decode()?;
                    img.copy_from(&read, x, 0)?;
                    x += read.width();
                }
                _ => (),
            }
        } else if path.is_dir() {
            compile(&path)?;
        }
    }

    Ok(img)
}

//Writes image to "compiledImages"
fn image_writer(img: DynamicImage, name: String) -> Result<(), Box<dyn Error>> {
    if !Path::new("./compiledImages").exists() {
        create_dir(Path::new("./compiledImages"))?;
    }

    img.save(format!("./compiledImages/{}.png", name))?;

    Ok(())
}
