use winit;
use winit::EventsLoop;
use winit::WindowBuilder;

use vulkano;
use vulkano::instance::Instance;
use vulkano_win::VkSurfaceBuild;

use std::sync::Arc;

pub struct Window {   
    pub width: u32,
    pub height: u32,
    pub title: String,
    events_loop: EventsLoop,
    window: Arc<vulkano::swapchain::Surface<winit::Window>>
}

impl Window{
    pub fn new(instance: Arc<Instance>) -> Window {      

        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new().with_dimensions((1024, 768).into()).with_title("Avis").build_vk_surface(&events_loop, instance).unwrap();

        Window {
            width: 1024,
            height: 768,
            title: String::from("Avis"),
            events_loop: events_loop,
            window: window
        }
    }

    pub fn run(&mut self) {
        self.events_loop.run_forever(|event| {
            match event {
                winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => {
                    winit::ControlFlow::Break
                },
                _ => winit::ControlFlow::Continue
            }
        });
    }
}