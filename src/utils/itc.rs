// Inter Thread Communication
use std::fmt;

#[derive(Debug)]
pub enum ITCStatus {
    Start,
    Tick,
    Wait,
    Shutdown,
    Error
}


impl fmt::Display for ITCStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
