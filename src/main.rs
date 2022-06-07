use std::fs::{self, remove_file};
use std::time::Instant;
use std::{error::Error, path::Path};

use rust_ic::compile;

fn main() -> Result<(), Box<dyn Error>> {
    if Path::new("./compiledImages").exists() {
        let paths = fs::read_dir(Path::new("./compiledImages")).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            remove_file(path)?;
        }
    }

    let start = Instant::now();
    compile(Path::new("YOUR PATH HERE"))?;
    let elapsed = start.elapsed();
    println!("Program took: {} ms", elapsed.as_millis());

    Ok(())
}
