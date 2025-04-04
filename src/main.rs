use std::{os::unix::net::UnixListener, sync::mpsc, thread};

use io_uring_sandbox::{backend::Backend, daemon::Daemon, filesystem::FileSystem};

// TODO: convert to command line args
const MAX_CHANNEL_SIZE: usize = 4096;
const SOCKET_PATH: &'static str = "./socket";

// TODO: improve error handling
fn main() {
    let (tx_async, mut rx_async) = tokio::sync::mpsc::channel(MAX_CHANNEL_SIZE);
    let (tx, rx) = mpsc::sync_channel(MAX_CHANNEL_SIZE);

    let listener = UnixListener::bind(SOCKET_PATH).unwrap();
    let fs = FileSystem::new();
    let backend = Backend::new(fs, tx_async);
    let mut daemon = Daemon::new(backend, rx);

    thread::spawn(move || {
        tokio_uring::start(async move {
            while let Some(req) = rx_async.recv().await {
                tokio_uring::spawn(async move {
                    let (server, req, stream) = req;
                    server.handle_message(req, stream).await;
                });
            }
        });
    });

    daemon.start(listener, tx);
    daemon.wait();
}
