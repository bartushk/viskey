use std::sync::mpsc;
use std::thread;
use super::{ LoggerBuilder, KeyLogger};


pub struct LinuxLogger {
    sender: mpsc::Sender<&'static str>,
}

pub struct LinuxKeyLoggerBuilder {

}


impl LoggerBuilder for LinuxKeyLoggerBuilder {
    fn new() -> (Box<KeyLogger>, mpsc::Receiver<&'static str>) {
        let (tx, rx) = mpsc::channel();
        (Box::new(LinuxLogger{sender: tx}), rx)
    }
}


impl KeyLogger for LinuxLogger {
    fn start_logging(&self) {
        let send_clone = self.sender.clone();
        thread::spawn( move || {
            for _ in 0..10 {
                //TODO: Actually log keys and send them up the channel.
                send_clone.send("asdf").unwrap();
                thread::sleep_ms(1000);
            }
        });
    }
}

