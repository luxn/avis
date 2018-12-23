pub mod vulkan;
pub mod window;

use vulkano;

use crate::utils::itc::ITCStatus;

use std::sync::Arc;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::thread;
use std::sync::mpsc::RecvError;
use std::sync::Mutex;


 fn run(receiver: mpsc::Receiver<ITCStatus>) -> Result<ITCStatus, RecvError> {
     println!("Starting");
     let mut counter = 0;
     loop {
         let res = receiver.recv()?;
         println!("{}, {:?}", counter, res);
         counter += 1;
         if counter >= 10 {
             break;
         }
     }
     println!("Ending");
     Ok(ITCStatus::Shutdown)
 }
pub struct RenderManager {
    vk_instance: Arc<vulkano::instance::Instance>,
    vk_device: Arc<vulkano::device::Device>,
    vk_queues: vulkano::device::QueuesIter,
    //receiver: mpsc::Receiver<ITCStatus>,
    thread_handle: JoinHandle<()>,
    running: Arc<Mutex<bool>>
}



impl RenderManager {
    pub fn new(recv: mpsc::Receiver<ITCStatus>) -> Self {

        let instance = vulkan::init_vulkan(); 
        println!("Hier2");


        let (device, mut queues) = vulkan::get_device_and_queues(instance.clone());

        vulkan::print_vulkan_debug_infos(instance.clone());

        //let recv_clone = recv.clone();

        let m = Arc::new(Mutex::new(true));
        let m_cloned = m.clone();
        let t = thread::spawn(move || {             
            let res = run(recv);
            let mut running = (*m_cloned).lock().unwrap();
            *running = false;
            println!("{:?}", res);
         });

        RenderManager {
            vk_instance: instance,
            vk_device: device,
            vk_queues: queues,
            //receiver: recv,
            thread_handle: t,
            running: m
        }
    }


    pub fn get_vk_instance(&self) -> Arc<vulkano::instance::Instance> {
        self.vk_instance.clone()
    }


    pub fn is_running(&self) -> bool {
        *(*self.running).lock().unwrap()
    }
   
}

