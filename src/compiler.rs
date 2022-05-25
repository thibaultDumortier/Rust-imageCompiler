use image::io::Reader;
use image::{open, GenericImageView, ImageBuffer, Rgba};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, create_dir, remove_file};
use std::ops::{Deref, DerefMut};
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
        let mut img = ImageBuffer::new(dim.0, dim.1);
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
                    let img = open(path).unwrap().into_rgba8();
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

fn compile_dir<C>(
    dir: &Path,
    mut img: ImageBuffer<Rgba<u8>, C>,
) -> Result<ImageBuffer<Rgba<u8>, C>, Box<dyn Error>>
where
    C: Deref<Target = [u8]> + DerefMut,
{
    assert!(dir.is_dir());
    let mut x = 0;
    let mut y = 0;
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let ext = path.extension().and_then(OsStr::to_str).unwrap();
            match ext {
                "png" | "bmp" | "jpeg" | "jpg" => {
                    let read = Reader::open(path)?.decode()?;
                    for i in 0..read.width() {
                        for j in 0..read.height() {
                            let pix = read.get_pixel(i, j);
                            img.put_pixel(x, y, pix);
                            y += 1;
                        }
                        y -= read.height();
                        x += 1;
                    }
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
// private void imageWriter(BufferedImage imageBuffer, String name){
fn image_writer<C>(img: ImageBuffer<Rgba<u8>, C>, name: String) -> Result<(), Box<dyn Error>>
where
    C: Deref<Target = [u8]> + DerefMut,
{
    if Path::new("./compiledImages").exists() == false {
        create_dir(Path::new("./compiledImages"))?;
    } else {
        let paths = fs::read_dir(Path::new("./compiledImages")).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            remove_file(path)?;
        }
    }

    img.save(format!("./compiledImages/{}.png", name))?;

    Ok(())
}
