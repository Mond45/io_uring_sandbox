use std::io;

use tokio_uring::fs::File;

pub struct FileSystem {}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {}
    }

    pub async fn read(&self, path: &str) -> io::Result<String> {
        // TODO: handle larger files
        let file = File::open(path).await?;
        let buf = vec![0; 4096];
        let (res, buf) = file.read_at(buf, 0).await;
        let n = res?;
        file.close().await?;
        Ok(std::str::from_utf8(&buf[..n]).unwrap().to_string())
    }

    pub async fn write(&self, path: &str, content: &str) -> io::Result<()> {
        let file = File::create(path).await?;
        let (res, _) = file.write_all_at(content.as_bytes().to_owned(), 0).await;
        res?;
        file.close().await?;
        Ok(())
    }
}
