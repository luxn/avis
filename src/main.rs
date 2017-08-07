extern crate cgmath;
extern crate glium;
extern crate rand;

mod graphics;
mod math;
mod scene;
mod utils;

use glium::{glutin, Surface};


use utils::file;

fn main() {   

    let result = file::load_shader_from_file("Hallo.welt".into());
    

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut closed = false;

    while !closed {
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(17));
    }
}
