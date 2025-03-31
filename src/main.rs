use std::{os::unix::net::UnixListener, sync::mpsc};

use io_uring_sandbox::{backend::Backend, daemon::Daemon, filesystem::FileSystem};

// TODO: convert to command line args
const MAX_CHANNEL_SIZE: usize = 4096;
const SOCKET_PATH: &'static str = "./socket";

// TODO: better error handling
fn main() {
    let listener = UnixListener::bind(SOCKET_PATH).unwrap();
    let fs = FileSystem::new();
    let backend = Backend::new(fs);
    let (tx, rx) = mpsc::sync_channel(MAX_CHANNEL_SIZE);
    let mut daemon = Daemon::new(backend, rx);

    daemon.start(listener, tx);
    daemon.wait();
}
