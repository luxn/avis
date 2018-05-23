extern crate rand;
//#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod graphics;
mod utils;

use std::{thread, time};
use std::sync::mpsc::channel;


fn main() {      

    let (sender, receiver) = channel();
   
   let renderer = graphics::RenderManager::new(receiver);

   sender.send(utils::itc::ITCStatus::Start);    

    let mut window = graphics::window::Window::new(renderer.get_vk_instance());

    println!("Window {}px x {}px with Name '{}'", window.width, window.height, window.title);
    
    let hundred_millis = time::Duration::from_millis(100);
    loop {
        println!("Sending Tick");
        sender.send(utils::itc::ITCStatus::Tick);
        thread::sleep(hundred_millis);

        //Health Check
        if !renderer.is_running() {
            break;
        }
    }
    //window.run();

}
