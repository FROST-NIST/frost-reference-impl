use thiserror_nostd_notrait::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Other(anyhow::Error, String),
}

fn main() {}
