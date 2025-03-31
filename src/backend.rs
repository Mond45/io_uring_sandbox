use std::io::prelude::*;
use std::os::unix::net::UnixStream;

use crate::{filesystem::FileSystem, server::Server, Request};

pub struct Backend {
    thread: BackendThread,
}

impl Backend {
    pub fn new(fs: FileSystem) -> Backend {
        Backend {
            thread: BackendThread::new(fs),
        }
    }

    pub(crate) fn handle_event(&self, stream: UnixStream) {
        self.thread.handle_event_serial(stream)
    }
}

struct BackendThread {
    server: Server,
}

impl BackendThread {
    fn new(fs: FileSystem) -> BackendThread {
        BackendThread {
            server: Server::new(fs),
        }
    }

    fn handle_event_serial(&self, mut stream: UnixStream) {
        // TODO: better error handling
        let mut s = String::new();
        stream.read_to_string(&mut s).unwrap();
        if let Ok(req) = serde_json::from_str::<Request>(&s) {
            self.server.handle_message(req, stream);
        }
    }
}
