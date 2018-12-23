extern crate rand;
//#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use std::sync::mpsc::channel;
use std::{thread, time};

mod utils;
mod graphics;



const fn foo(x: i32) -> i32 {
    x + 1
}

const SIX: i32 = foo(5);

fn main() {
    println!("Hello World!");

    let (sender, receiver) = channel();

    let renderer = graphics::RenderManager::new(receiver);

    sender.send(utils::itc::ITCStatus::Start).unwrap();

    let mut window = graphics::window::Window::new(renderer.get_vk_instance());

    println!(
        "Window {}px x {}px with Name '{}'",
        window.width, window.height, window.title
    );

    let hundred_millis = time::Duration::from_millis(100);
    loop {
        println!("Sending Tick");
        sender.send(utils::itc::ITCStatus::Tick).unwrap();
        thread::sleep(hundred_millis);

        //Health Check
        if !renderer.is_running() {
            break;
        }
    }
    window.run();
}
