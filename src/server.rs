use std::{io::Write, os::unix::net::UnixStream};

use crate::{filesystem::FileSystem, Request};

pub struct Server {
    fs: FileSystem,
}

impl Server {
    pub fn new(fs: FileSystem) -> Server {
        Server { fs }
    }

    pub async fn handle_message(&self, req: Request, mut stream: UnixStream) {
        match req {
            // TODO: improve error handling
            Request::Read { path } => {
                stream
                    .write_all(self.fs.read(&path).await.unwrap().as_bytes())
                    .unwrap();
            }
            Request::Write { path, content } => {
                self.fs.write(&path, &content).await.unwrap();
                stream.write_all(b"OK").unwrap();
            }
        }
        // quite ugly, but otherwise read_to_string will hang
        stream.shutdown(std::net::Shutdown::Write).unwrap();
    }
}
