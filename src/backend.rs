use std::os::unix::net::UnixStream;
use std::{io::prelude::*, sync::Arc};

use tokio::sync::mpsc;

use crate::{filesystem::FileSystem, server::Server, Request};

pub struct Backend {
    thread: BackendThread,
}

type Sender = mpsc::Sender<(Arc<Server>, Request, UnixStream)>;

impl Backend {
    pub fn new(fs: FileSystem, sender: Sender) -> Backend {
        Backend {
            thread: BackendThread::new(fs, sender),
        }
    }

    pub(crate) fn handle_event(&self, stream: UnixStream) {
        self.thread.handle_event_async(stream)
    }
}

struct BackendThread {
    server: Arc<Server>,
    sender: Sender,
}

impl BackendThread {
    fn new(fs: FileSystem, sender: Sender) -> BackendThread {
        BackendThread {
            server: Arc::new(Server::new(fs)),
            sender,
        }
    }

    fn handle_event_async(&self, mut stream: UnixStream) {
        // TODO: improve error handling
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();
        if let Ok(req) = serde_json::from_str::<Request>(&buf) {
            self.sender
                .blocking_send((self.server.clone(), req, stream))
                .unwrap();
        }
    }
}
