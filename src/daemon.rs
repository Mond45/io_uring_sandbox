use std::{
    os::unix::net::{UnixListener, UnixStream},
    sync::mpsc,
    thread,
};

use crate::backend::Backend;

pub struct Daemon {
    main_thread: Option<thread::JoinHandle<()>>,
}

impl Daemon {
    pub fn new(backend: Backend, receiver: mpsc::Receiver<UnixStream>) -> Daemon {
        // spawn a daemon handler thread (corresponds to VhostUserHandler)
        thread::spawn(move || {
            while let Ok(stream) = receiver.recv() {
                backend.handle_event(stream);
            }
        });
        Daemon { main_thread: None }
    }

    // TODO: returns Result
    pub fn start(&mut self, listener: UnixListener, sender: mpsc::SyncSender<UnixStream>) {
        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        sender.send(stream).unwrap();
                    }
                    Err(_) => break,
                }
            }
        });
        self.main_thread = Some(handle);
    }

    // TODO: returns Result
    pub fn wait(&mut self) {
        if let Some(handle) = self.main_thread.take() {
            handle.join().unwrap();
        }
    }
}
