use super::{Player, PlayerError};
use crate::music::*;
use rodio::{OutputStreamHandle, Sink};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread::{self, JoinHandle};

pub enum ThreadMessage {
    Interrupt,
    Abort,
}

/// A representation of a threaded instance of [Player]
#[derive(Debug)]
pub struct PlayerThread {
    handle: JoinHandle<Player>,
    rx: Receiver<()>,
    tx: Sender<ThreadMessage>,
}

impl PlayerThread {
    /// Consumes [Player] and immediately starts playing music.
    pub fn new(
        mut player: Player,
        stream: &OutputStreamHandle,
        music: Music,
    ) -> Result<PlayerThread, PlayerError> {
        let (player_tx, rx) = mpsc::channel();
        let (tx, interrupt_rx) = mpsc::channel();
        player.rx = Some(interrupt_rx);
        let sink = match Sink::try_new(stream) {
            Ok(sink) => sink,
            Err(_) => return Err(PlayerError::ThreadError),
        };
        let handle = thread::spawn(move || {
            player.play(music, sink);
            player_tx.send(()).ok();
            player
        });
        Ok(PlayerThread { handle, rx, tx })
    }

    /// Indicates whether the [Player] has finished playing music and is
    /// ready for [PlayerThread::join], returns an error if the thread
    /// has become disconnected. This only returns true once, so the
    /// thread must be immediately joined.
    pub fn finished_playing(&self) -> Result<bool, PlayerError> {
        match self.rx.try_recv() {
            Ok(()) => Ok(true),
            Err(TryRecvError::Empty) => Ok(false),
            Err(TryRecvError::Disconnected) => Err(PlayerError::ThreadError),
        }
    }

    /// Sends a message to the thread to interrupt a pause.
    /// Returns an error if the thread has become disconnected.
    pub fn interrupt(&self) -> Result<(), PlayerError> {
        match self.tx.send(ThreadMessage::Interrupt) {
            Ok(()) => Ok(()),
            Err(_) => Err(PlayerError::ThreadError),
        }
    }

    /// Sends a message to the thread to abort playing music.
    /// Returns an error if the thread has become disconnected.
    pub fn abort(&self) -> Result<(), PlayerError> {
        match self.tx.send(ThreadMessage::Abort) {
            Ok(()) => Ok(()),
            Err(_) => Err(PlayerError::ThreadError),
        }
    }

    /// Attempts to join the spawned thread and returns the [Player]
    /// state if succesful.
    /// Returns an error if the thread has become disconnected.
    pub fn join(self) -> Result<Player, PlayerError> {
        match self.handle.join() {
            Ok(player) => Ok(player),
            Err(_) => Err(PlayerError::ThreadError),
        }
    }
}
