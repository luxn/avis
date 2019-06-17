use std::fs::File;
use std::io::Read;
use std::str;

use gl;
use gl::types::*;
use std::ffi::{CString, CStr};
use std::{ptr, fmt};
use std::process::exit;
use crate::math::Vector3f;

pub struct ShaderProgram {
    program_id: u32,
    vertex_shader_id: u32,
    fragment_shader_id: u32,
}

#[derive(Copy,Clone)]
pub enum ShaderProgramType {
    Vertex,
    Fragment
}

impl fmt::Display for ShaderProgramType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ShaderProgramType::Vertex => "VERTEX",
            ShaderProgramType::Fragment => "FRAGMENT",
        };
        write!(f, "{}", printable)
    }
}



impl ShaderProgram {

    fn new(vertex_file: String, fragment_file: String) -> ShaderProgram {
        let vertex_shader_id = ShaderProgram::load_shader(vertex_file, ShaderProgramType::Vertex);
        let fragment_shader_id = ShaderProgram::load_shader(fragment_file, ShaderProgramType::Fragment);

        unsafe {
            let program_id = gl::CreateProgram();

            let instance = ShaderProgram {
                program_id,
                vertex_shader_id,
                fragment_shader_id
            };

            gl::AttachShader(program_id, vertex_shader_id);
            gl::AttachShader(program_id, fragment_shader_id);

            instance.bind_attributes();

            gl::LinkProgram(program_id);

            // check for linking errors
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(program_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
                exit(-1);
            }

            instance.get_all_uniform_locations();

            instance
        }
    }

    fn get_uniform_location(&self, uniform_name: &CString) -> u32 {
        unsafe {
            gl::GetUniformLocation(self.program_id, uniform_name.as_ptr()) as u32
        }
    }

    fn get_all_uniform_locations(&self) {

    }

    pub fn start(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn stop(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    fn bind_attributes(&self) {
        self.bind_attribute(0, &CString::new("position").unwrap());
        self.bind_attribute(1, &CString::new("textureCoords").unwrap());

    }

    fn bind_attribute(&self, attribute: u32, variable_name: &CString) {
        unsafe {
            gl::BindAttribLocation(self.program_id, attribute, variable_name.as_ptr());
        }
    }

    fn load_float(location: i32, value: f32) {
        unsafe {
            gl::Uniform1f(location, value);
        }
    }

    fn load_vector(location: i32, vector: &Vector3f) {
        unsafe {
            gl::Uniform3f(location, vector.x, vector.y, vector.z);
        }
    }

    fn load_bool(location: i32, value: bool) {
        unsafe {
            if value {
                gl::Uniform1f(location, 1.0f32);
            } else{
                gl::Uniform1f(location, 0.0f32);
            }
        }
    }

    //fn load_matrix(location: u32, value: Matrix4f) {
    //    unsafe {
    //        //gl::UniformMatrix4(location, false, )
    //    }
    //}


    fn load_shader(file: String, shader_type: ShaderProgramType) -> u32 {
        let shader_data = {
            let mut file = File::open(file).unwrap();
            let mut buffer = String::new();
            file.read_to_string( & mut buffer).unwrap();
            buffer
        };

        let shader_data_cstr = CString::new(shader_data.as_bytes()).unwrap();

        let gl_shader_type = match shader_type {
            ShaderProgramType::Vertex => gl::VERTEX_SHADER,
            ShaderProgramType::Fragment => gl::FRAGMENT_SHADER,
        };

        unsafe {
            let shader_id = gl::CreateShader(gl_shader_type);
            gl::ShaderSource(shader_id, 1, &shader_data_cstr.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);

            // check for shader compile errors
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::{}::COMPILATION_FAILED\n{}", shader_type, str::from_utf8(&info_log).unwrap());
                exit(-1);
            }

            return shader_id;
        }

    }
}


impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.stop();
        unsafe {
            gl::DetachShader(self.program_id, self.vertex_shader_id);
            gl::DetachShader(self.program_id, self.vertex_shader_id);
            gl::DeleteShader(self.vertex_shader_id);
            gl::DeleteShader(self.fragment_shader_id);
            gl::DeleteProgram(self.program_id);
        }

    }
}


pub fn create_static_shader() -> ShaderProgram {
    let vert = String::from("res/shaders/vertexShader.txt");
    let frag = String::from("res/shaders/fragmentShader.txt");

    ShaderProgram::new(vert, frag)
}