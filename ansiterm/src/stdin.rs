use std::io::{stdin, Read};
pub use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

pub fn spawn_stdin_channel() -> Receiver<Vec<u8>> {
    let (tx, rx) = channel();
    let mut buffer = vec![0; 1024 * 1024];
    let one_frame_duration = Duration::from_millis(1000 / 60);
    thread::spawn(move || loop {
        match stdin().read(buffer.as_mut_slice()) {
            Ok(0) => thread::sleep(one_frame_duration),
            Ok(len) => {
                let bytes = buffer[..len].to_vec();
                if tx.send(bytes).is_err() {
                    break;
                }
            }
            Err(_) => break,
        };
    });
    rx
}
