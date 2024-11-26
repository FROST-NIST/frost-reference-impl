use thiserror_nostd_notrait::Error;

#[derive(Error, Debug)]
#[error("...")]
#[error("...")]
pub struct Error;

fn main() {}
