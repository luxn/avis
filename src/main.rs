extern crate rand;
#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod graphics;

fn main() {      

    let instance = graphics::vulkan::init_vulkan();

    graphics::vulkan::print_vulkan_debug_infos(instance.clone());

    let mut window = graphics::window::Window::new(instance.clone());

    println!("Window {}px x {}px with Name '{}'", window.width, window.height, window.title);

    window.run();

}
