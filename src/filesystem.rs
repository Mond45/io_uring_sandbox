pub struct FileSystem {}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {}
    }

    pub fn read(&self, path: &str) -> std::io::Result<String> {
        std::fs::read_to_string(path)
    }

    pub fn write(&self, path: &str, content: &str) -> std::io::Result<()> {
        std::fs::write(path, content)?;
        Ok(())
    }
}
