#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod messages;
mod utils;

pub use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};

#[tokio::main]
async fn main() -> IOResult<()> {
    println!("Hello, world!");

    Ok(())
}
