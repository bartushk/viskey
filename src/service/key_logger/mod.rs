//! The key_logger module is an abstraction allowing for easy interaction raw user keyboard input.
//!
//! This module describes the interface for a KeyLogger and the Builder used to create a KeyLogger.
//! It does not contain any concrete implementations itself, but instead exposes modules that
//! contain implementations for each supported system.
//!
//! #Examples
//! 
//! let (logger, receiver) = LinuxLoggerBuilder::new();
//! logger.start_logging();
//! loop {
//!     let received_key = receiver.recv().unwrap();
//! }
//!

pub mod linux_key_logger;
use std::sync::mpsc;
use std::io::Error;

#[derive(Debug, PartialEq)]
pub enum KeyAction {
    Up,
    Down,
    Held,
    Unknown,
}

#[derive(Debug)]
pub struct KeyPress {
    action: KeyAction,
    value: &'static str,
}

/// Describes the abstracted functionality of a keylogger.
///
/// #Examples
///
/// let (logger, receiver) = LinuxLoggerBuilder::new();
/// logger.start_logging();
/// loop {
///     let received_key = receiver.recv().unwrap();
/// }
///
pub trait KeyLogger {
    fn start_logging(&self) -> Vec<Result<&str, Error>>;
}

/// Trait for describing the construction of a logger.
///
/// This trait will be implemented by each platform that will support viskey.
///
/// #Examples
///
/// let (logger, receiver) = LinuxLoggerBuilder::new();
/// logger.start_logging();
/// loop {
///     let received_key = receiver.recv().unwrap();
/// }
///
pub trait LoggerBuilder {
    fn new() -> (Box<KeyLogger>, mpsc::Receiver<KeyPress>);
}
