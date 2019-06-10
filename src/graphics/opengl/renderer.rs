use crate::graphics::opengl::RawModel;
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


    pub fn render(&self, model: &RawModel) {
        unsafe {
            gl::BindVertexArray(model.vao_id());
            gl::EnableVertexAttribArray(0);
            gl::DrawElements(gl::TRIANGLES, model.vertex_count() as GLsizei, gl::UNSIGNED_INT, ptr::null());
            gl::DisableVertexAttribArray(0);
            gl::BindVertexArray(0);
        }
    }
}