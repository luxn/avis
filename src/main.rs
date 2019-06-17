extern crate gl;
extern crate glfw;
extern crate image;

mod graphics;
mod input;
mod math;
mod scene;
mod utils;


use glfw::{Action, Context, Key};
use std::mem;
use gl::types::*;
use std::os::raw::c_void;
use std::ptr;

use crate::graphics::opengl::{TexturedModel, ModelTexture, Renderer, Loader, create_static_shader};


fn load_attribute(name: &'static str, value: f32) {
    // TODO
}


const PI: f32 = std::f32::consts::PI;

fn main() {


    load_attribute("Test", PI);

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));


    let (mut window, events) = glfw.create_window(1024, 768, "Hello this is window!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut loader = Loader::new();
    let renderer = Renderer::new();

    let vertices: Vec<f32> = vec![
        -0.5, 0.5, 0.0,       //v0
        -0.5, -0.5, 0.0,      //v1
        0.5, -0.5, 0.0,       //v2
        0.5, 0.5, 0.0,        //v3
    ];

    let indices: Vec<u32> = vec![
        0,1,3,  //top left triangle (v0, v1, v3)
        3,1,2   //bottom right triangle (v3, v1, v2)
    ];

    let texture_coordinates = vec![
        1.0, 0.0,    //v0
        0.0, 0.0,    //v1
        0.0, 1.0,    //v2
        1.0, 1.0,    //v3
    ];


    let textured_model = {
        let model = loader.load_to_vao(&vertices, &texture_coordinates, &indices);
        let texture_id = loader.load_texture("res/textures/smiley.png");
        let texture = ModelTexture::new(texture_id);
        TexturedModel::new(model, texture)
    };

    let shader = create_static_shader();

    while !window.should_close() {
        window.swap_buffers();

        renderer.prepare();
        shader.start();

        renderer.render(&textured_model);

        shader.stop();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                _ => {},
            }
        }
    }
}

