pub mod service;

use service::key_logger::linux_key_logger::LinuxKeyLoggerBuilder;
use service::key_logger::LoggerBuilder;

fn main() {
    let (logger, channel) = LinuxKeyLoggerBuilder::new();
    logger.start_logging();
    for _ in 0..10 {
        let result = channel.recv().unwrap();
        println!("{}", result);
    }
}
