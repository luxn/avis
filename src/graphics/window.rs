use glium;
use glium::glutin;

pub struct Window {
    events_loop: glutin::EventsLoop,   
    display: glium::Display
}

impl Window{
    fn new() -> Window{
        let mut events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new();
        let context_builder = glutin::ContextBuilder::new();
        let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

        Window {
            events_loop: events_loop,          
            display: display
        }
    }
}