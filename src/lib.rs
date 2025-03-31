use serde::{Deserialize, Serialize};

pub mod backend;
pub mod daemon;
pub mod filesystem;
pub mod server;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Read { path: String },
    Write { path: String, content: String },
}
