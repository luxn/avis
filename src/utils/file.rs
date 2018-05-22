use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn load_shader_from_file(path: &str) -> Result<String, std::io::Error> {
    let file = match File::open(format!("shaders/{}.glsl", path)) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return Err(e);
        }
    };
    let mut buffer = String::new()
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
} 

