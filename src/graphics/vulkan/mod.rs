use vulkano::instance::Instance;
//use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::QueuesIter;
use vulkano::instance::Features;
use vulkano::device::DeviceExtensions;

use vulkano_win;

use std::sync::Arc;

pub fn init_vulkan() -> Arc<Instance> {
    let instance: Arc<Instance> = {        
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create vkInstance")       
    };
    instance
}

pub fn print_vulkan_debug_infos(instance: Arc<Instance>) {
    for physical_device in PhysicalDevice::enumerate(&instance) {
        println!("Name: {}", physical_device.name());
        println!("API Version: {}", physical_device.api_version());
        println!("Driver Version: {}", physical_device.driver_version());
        println!("Device Type: {:?}", physical_device.ty());


        println!("Count of QueueFamilies: {}", physical_device.queue_families().len());
        for queue in physical_device.queue_families() {
            println!("\tQueueFamily {} (Num of Queues: {}) supports graphics ({}) and compute ({})", 
                queue.id(), 
                queue.queues_count(),
                queue.supports_graphics(), 
                queue.supports_compute()
            );
        }
    }
}

pub fn get_device_and_queues(instance: Arc<Instance>) -> (Arc<Device>, QueuesIter) {

    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();

    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    //0.5 is a priority for the given queue_family
    Device::new(physical, &Features::none(), &DeviceExtensions::none(),
                [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
}