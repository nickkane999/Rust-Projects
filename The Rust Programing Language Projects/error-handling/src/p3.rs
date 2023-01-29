use std::fs::{self, File};
use std::io;
use std::io::Read;

pub fn run() -> Result<String, io::Error> {

    /* 
    let f = File::open("hello.txt")?;
    let mut f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => println!("s = {}", s),
        Err(error) => panic!("Problem reading file: {:?}", error),
    }    
    */

    /*
    let mut f = File::open("hello.txtttt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    
    */

    /*
    let mut s = String::new();
    File::open("hello.txtttt")?.read_to_string(&mut s)?;
    Ok(s)
    
    */
    fs::read_to_string("hello.textt")


}