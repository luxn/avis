use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Error;


pub fn load_shader_from_file(path: &str) -> Result<String, Error> {
    let file = match File::open(format!("shaders/{}.glsl", path)) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return Err(e);
        }
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
} 

