use std::fs::File;
//use std::io::BufReader;
use std::io::prelude::*;
use std::io;


pub fn load_shader_from_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(format!("shaders/{}.glsl", path))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
} 

