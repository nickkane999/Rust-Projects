use std::fs::File; 
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}