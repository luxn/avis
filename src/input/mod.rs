use std::thread::Thread;
use std::sync::mpsc;
use winit;

struct Input {
    thread_handle: Thread,
    events_loop: winit::EventsLoop
}
