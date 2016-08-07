extern crate timebomb;

use std::sync::mpsc;
use std::thread;

use self::timebomb::timeout_ms;

use super::{ LoggerBuilder, KeyLogger};



pub struct LinuxLogger {
    sender: mpsc::Sender<&'static str>,
}

pub struct LinuxLoggerBuilder {

}


impl LoggerBuilder for LinuxLoggerBuilder {
    fn new() -> (Box<KeyLogger>, mpsc::Receiver<&'static str>) {
        let (tx, rx) = mpsc::channel();
        (Box::new(LinuxLogger{sender: tx}), rx)
    }
}


impl KeyLogger for LinuxLogger {
    fn start_logging(&self) {
        let send_clone = self.sender.clone();
        thread::spawn( move || {
            send_clone.send("Logging started.").unwrap();
            //TODO: Actually log keys and send them up the channel.
        });
    }
}


#[test]
fn start_logging_message_received() {
    timeout_ms(|| {
        let (logger, rx) =  LinuxLoggerBuilder::new();
        logger.start_logging();
        let result = rx.recv().unwrap();
        assert_eq!(result, "Logging started.");
    }, 1000);
}
