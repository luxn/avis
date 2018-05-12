extern crate rand;

#[macro_use]
extern crate vulkano;

extern crate vulkano_win;

extern crate winit;


use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;

use vulkano_win::VkSurfaceBuild;

use winit::EventsLoop;
use winit::WindowBuilder;

fn main() {   
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create vkInstance")
    };

    let mut events_loop = EventsLoop::new();

    let window = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => {
                winit::ControlFlow::Break
            },
            _ => winit::ControlFlow::Continue
        }
    });

}


/*
mod graphics;
mod math;
mod scene;
mod utils;
*/



