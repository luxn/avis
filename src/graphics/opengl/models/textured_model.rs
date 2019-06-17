use crate::graphics::opengl::{RawModel, ModelTexture};
use core::borrow::Borrow;

pub struct TexturedModel {
    raw_model: RawModel,
    texture: ModelTexture,
}



impl TexturedModel {

    pub fn new(model: RawModel, texture: ModelTexture) -> Self {
        TexturedModel {
            raw_model: model,
            texture: texture
        }
    }

    pub fn raw_model(&self) -> &RawModel {
        return &self.raw_model
    }

    pub fn texture(&self) -> &ModelTexture {
        return &self.texture()
    }
}