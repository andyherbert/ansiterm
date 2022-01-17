/*! A structure to help read from stdin in a separate thread, and a method to halt execution. */
mod error;
pub use error::StdInReceiverError;
use std::{
    io::{self, Read},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread::{self, JoinHandle},
    time,
};

/// A structure which provides an entrypoint to receive and halt a thread to read from stdin.
pub struct StdInReceiver {
    vec_rx: mpsc::Receiver<Vec<u8>>,
    abort_tx: mpsc::SyncSender<()>,
    handle: JoinHandle<Result<(), StdInReceiverError>>,
}

impl Default for StdInReceiver {
    /// Constructs a [StdInReceiver] with a `buffer_len` of 1024 bytes.
    fn default() -> Self {
        StdInReceiver::new(1024)
    }
}

fn read(
    buffer_len: usize,
    vec_tx: mpsc::Sender<Vec<u8>>,
    abort_rx: Receiver<()>,
) -> Result<(), StdInReceiverError> {
    let mut buf: Vec<u8> = vec![0; buffer_len];
    let dur = time::Duration::from_millis(1000 / 60);
    loop {
        match io::stdin().read(&mut buf[..]) {
            Ok(0) => thread::sleep(dur),
            Ok(len) => {
                let bytes = buf[..len].to_vec();
                if vec_tx.send(bytes).is_err() {
                    return Err(StdInReceiverError::UnableToSend);
                }
            }
            Err(_) => return Err(StdInReceiverError::UnableToReadFromStdIn),
        }
        match abort_rx.try_recv() {
            // Recieved the signal to break.
            Ok(()) => return Ok(()),
            Err(TryRecvError::Empty) => continue,
            Err(TryRecvError::Disconnected) => return Err(StdInReceiverError::ThreadDisconnected),
        }
    }
}

impl StdInReceiver {
    /// Defines a new [StdInReceiver] with a buffer size of `buffer_len` bytes.
    pub fn new(buffer_len: usize) -> StdInReceiver {
        let (vec_tx, vec_rx) = mpsc::channel();
        let (abort_tx, abort_rx) = mpsc::sync_channel(0);
        let handle = thread::spawn(move || read(buffer_len, vec_tx, abort_rx));
        StdInReceiver {
            vec_rx,
            abort_tx,
            handle,
        }
    }

    /// Either receives an [Option] of none or some [Vec] of [u8], or results in an [StdInReceiverError].
    pub fn recv(&self) -> Result<Option<Vec<u8>>, StdInReceiverError> {
        match self.vec_rx.try_recv() {
            Ok(vec) => Ok(Some(vec)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(StdInReceiverError::ThreadDisconnected),
        }
    }

    /// Attempts to halt the thread and receives an error generated when attempting
    /// to halt the thread, or during the thread's execution.
    pub fn join(self) -> Result<(), StdInReceiverError> {
        match self.abort_tx.send(()) {
            Ok(()) => match self.handle.join() {
                Ok(result) => result,
                Err(_) => Err(StdInReceiverError::UnableToJoin),
            },
            Err(_) => Err(StdInReceiverError::UnableToSend),
        }
    }
}
