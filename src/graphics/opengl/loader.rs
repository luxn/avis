use crate::graphics::opengl::*;
use std::{mem, ptr};
use std::ffi::c_void;

use gl;
use gl::types::*;

use image;
use image::{GenericImageView, DynamicImage};
use std::any::Any;


pub struct Loader {
    vaos: Vec<u32>,
    vbos: Vec<u32>,
    textures: Vec<u32>,
    images: Vec<DynamicImage>
}

impl Loader {


    pub fn new() -> Self {
        Loader {
            vaos: Vec::new(),
            vbos: Vec::new(),
            textures: Vec::new(),
            images: Vec::new()
        }
    }


    pub fn load_to_vao(&mut self, positions: &Vec<f32>, texture_coords: &Vec<f32>, indices: &Vec<u32>) -> RawModel {
        let vao_id = self.create_vao();
        self.bind_indices_buffer(indices);

        self.store_data_in_attribute_list(0, 3, positions);
        self.store_data_in_attribute_list(1, 2, texture_coords);

        self.unbind_vao();
        return RawModel::new(vao_id, indices.len() as u32);
    }


    pub fn load_texture(&mut self, file_name: &str) -> u32 {
        let mut img = image::open(file_name).expect("Unable to load Texture");

        let mut texture_id = 0;
        let data = match img {
            DynamicImage::ImageLuma8(ref a) => a.as_ptr(),

            DynamicImage::ImageLumaA8(ref a) => a.as_ptr(),

            DynamicImage::ImageRgb8(ref a) => a.as_ptr(),

            DynamicImage::ImageRgba8(ref a) => a.as_ptr(),

            DynamicImage::ImageBgr8(ref a) => a.as_ptr(),

            DynamicImage::ImageBgra8(ref a) => a.as_ptr(),

            _ => { panic!("unsupported image type") }
        };

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D, 0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data as *const c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        self.textures.push(texture_id);

        texture_id
    }

    fn create_vao(&mut self) -> u32 {
        unsafe {
            let mut vao_id = 0;
            gl::GenVertexArrays(1, &mut vao_id);
            self.vaos.push(vao_id);
            gl::BindVertexArray(vao_id);
            return vao_id;
        }
    }

    fn store_data_in_attribute_list(&mut self, attribute_number: u32, coordinate_size: i32, data: &Vec<f32>) {
        unsafe {
            let mut vbo_id = 0;
            gl::GenBuffers(1, &mut vbo_id);
            self.vbos.push(vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &data[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW);
            gl::VertexAttribPointer(attribute_number, coordinate_size, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn bind_indices_buffer(&mut self, indices: &Vec<u32>) {
        unsafe {
            let mut vbo_id = 0;
            gl::GenBuffers(1, &mut vbo_id);
            self.vbos.push(vbo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                           &indices[0] as *const u32 as *const c_void,
                           gl::STATIC_DRAW);

        }
    }

    fn unbind_vao(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Loader {
    fn drop(&mut self) {
        unsafe {
            for vao in self.vaos.iter() {
                gl::DeleteVertexArrays(1, vao);
            }

            for vbo in self.vbos.iter() {
                gl::DeleteBuffers(1, vbo);
            }

            for texture in self.textures.iter() {
                gl::DeleteTextures(1, texture);
            }
        }
    }
}