use crate::graphics::opengl::{RawModel, TexturedModel};
use std::ptr;

use gl;
use gl::types::*;

pub struct Renderer {}

impl Renderer {

    pub fn new() -> Self {
        Renderer {}
    }


    pub fn prepare(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
        }
    }


    pub fn render(&self, textured_model: &TexturedModel) {
        let model = textured_model.raw_model();
        unsafe {
            gl::BindVertexArray(model.vao_id());

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

            //gl::ActiveTexture(gl::TEXTURE0);
            //gl::BindTexture(gl::TEXTURE_2D, textured_model.texture().id());

            gl::DrawElements(gl::TRIANGLES, model.vertex_count() as GLsizei, gl::UNSIGNED_INT, ptr::null());

            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(0);

            gl::BindVertexArray(0);
        }
    }
}