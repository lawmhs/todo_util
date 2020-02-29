#![feature(with_options)]
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// done without structopt for now, so that we can get an idea of what structops does for us
fn main() -> std::io::Result<()> {

    let mut file;

    // create the basic todo stack if it doesn't currently exist
    if !Path::new("stack").exists() {
        File::create("stack")
                    .expect("failed to create new stack file");
    }
    file = File::with_options()
                    .read(true)
                    .write(true)
                    .open("stack")
                    .expect("existing stack file failed to open");

    // read the file contents into a string to work with it
    let mut stack_contents = String::new();
    file.read_to_string(&mut stack_contents).expect("unable to read stack");
    
    // read from the first argument
    let mut pattern = std::env::args().nth(1).expect("no pattern given");
    
    // then append the new pattern to our new stack : 
    pattern.push_str("\n");
    
    file.write_all(pattern.as_bytes())?;
    Ok(())
}
