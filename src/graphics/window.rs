use std::sync::Arc;
use std::sync::mpsc::Sender;

use crate::input::InputEvent;

pub struct Window {   
    pub width: u32,
    pub height: u32,
    pub title: String,
    sender: Sender<InputEvent>
}

impl Window{

    pub fn new(event_sender: Sender<InputEvent>) -> Window {
        Window {
            width: 1024,
            height: 768,
            title: String::from("Avis"),
            sender: event_sender
        }
    }

    pub fn run(&mut self) {
        let cb_sender = self.sender.clone();
        loop {

        }
    }
}