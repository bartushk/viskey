use std::sync::mpsc;
use std::thread;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;

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
    fn start_logging(&self) -> Vec<Result<&str, Error>> {
        let mut results : Vec<Result<&str, Error>> = vec!(); 
        let mut device_str = String::new();
        match File::open("/proc/bus/input/devices"){
            Ok(mut file) => {
                match file.read_to_string(&mut device_str) {
                    Ok(_) => (),
                    Err(err) => results.push(Err(err)),
                }
            },
            Err(err) => results.push(Err(err)),
        };

        if results.len() > 0 {
            return results;
        }

        for path in get_kbd_device_paths(device_str.as_str()) {
            let send_clone = self.sender.clone();
            println!("Path: {}", path);
            thread::spawn( move || {
                send_clone.send("Logging started.").unwrap();
                //TODO: Actually log keys and send them up the channel.
            });
        }

        results
    }
}



fn get_kbd_device_paths(device_str: &str) -> Vec<String> {
    //TODO: Regex these values from string of /proc/bus/input/devices
    println!("Device str: \n{}", device_str);
    vec!["/dev/input/event4".to_string(), "/dev/input/event2".to_string()] 
}



#[cfg(test)]
mod tests {

    extern crate timebomb;
    use self::timebomb::timeout_ms;
    use super::*;
    use super::super::*;

    #[test]
    fn start_logging_message_received() {
        timeout_ms(|| {
            let (logger, rx) =  LinuxLoggerBuilder::new();
            logger.start_logging();
            let result = rx.recv().unwrap();
            assert_eq!(result, "Logging started.");
        }, 1000);
    }

}
