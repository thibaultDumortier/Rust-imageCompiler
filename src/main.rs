use std::{error::Error, path::Path};

use rust_ic::compile;

fn main() -> Result<(), Box<dyn Error>> {
    compile(Path::new("YOUR PATH HERE"))?;
    Ok(())
}
