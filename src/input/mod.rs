use std::thread::Thread;
use std::sync::mpsc;
use std::fmt;
//use winit;

pub enum KeyCode {
    A,
    B,
    C,
    D
}


impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            A => write!(f, "KeyCode(A)"),
            B => write!(f, "KeyCode(B)"),
            _ => write!(f, "KeyCode(..)")
        }

    }
}

pub enum MouseKeyCode {
    Left,
    Middle,
    Right
}


pub enum WindowEvent {
    WindowFocused(bool),
    WindowClosed
}

pub enum KeyboardEvent {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode)
}

pub enum MouseEvent {
    MouseMoved(i32, i32),
    MouseButtonPressed(MouseKeyCode),
    MouseButtonReleased(MouseKeyCode),
    MouseScrolled(i32)
}

pub enum InputEvent {
    WindowEvent(WindowEvent),
    KeyboardEvent(KeyboardEvent),
    MouseEvent(MouseEvent),
    Unknown
}


/*
impl std::convert::From<winit::Event> for InputEvent {
    fn from(event: winit::Event) -> Self {
        return match event {
            winit::Event::WindowEvent { event, .. } => {
                match event {
                    winit::WindowEvent::CloseRequested => {
                        InputEvent::WindowEvent(WindowEvent::WindowClosed)
                    },
                    winit::WindowEvent::Focused(focused) => {
                        InputEvent::WindowEvent(WindowEvent::WindowFocused(focused))
                    },
                    _ => InputEvent::Unknown
                }
            },
            winit::Event::DeviceEvent { event, ..} => {
                match event {
                    winit::DeviceEvent::Key(key) => {
                        InputEvent::KeyboardEvent(KeyboardEvent::KeyPressed(KeyCode::A))
                    },
                    _ => InputEvent::Unknown
                }
            }
            _ => {
                InputEvent::Unknown
            }
        };
    }
}
*/