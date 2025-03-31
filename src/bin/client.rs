use std::io::prelude::*;
use std::os::unix::net::UnixStream;

use io_uring_sandbox::Request;

fn main() {
    let reqs = vec![
        Request::Write {
            path: "./test.txt".to_string(),
            content: "The quick brown fox jumps over the lazy dog.".to_string(),
        },
        Request::Read {
            path: "./test.txt".to_string(),
        },
    ];

    for req in reqs.into_iter() {
        let mut stream = UnixStream::connect("./socket").unwrap();
        stream
            .write_all(serde_json::to_string(&req).unwrap().as_bytes())
            .unwrap();
        // quite ugly, but otherwise read_to_string will hang
        stream.shutdown(std::net::Shutdown::Write).unwrap();
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();
        println!("{buf}");
    }
}
