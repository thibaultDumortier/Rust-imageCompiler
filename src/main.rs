use std::fs::{self, remove_file};
use std::time::Instant;
use std::{error::Error, path::Path};

use rust_ic::compile;

fn main() -> Result<(), Box<dyn Error>> {
    clear()?;

    let start = Instant::now();
    compile(Path::new("to_compile/"))?; //Change path then run "cargo run --release"
    let elapsed = start.elapsed();
    println!("Program took: {} ms", elapsed.as_millis());

    //You can add a decompiler here if you so wish

    Ok(())
}

fn clear() -> Result<(), Box<dyn Error>> {
    if Path::new("./compiledImages").exists() {
        let paths = fs::read_dir(Path::new("./compiledImages")).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            remove_file(path)?;
        }
    }
    if Path::new("./decompiledImages").exists() {
        let paths = fs::read_dir(Path::new("./decompiledImages")).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            remove_file(path)?;
        }
    }
    Ok(())
}
