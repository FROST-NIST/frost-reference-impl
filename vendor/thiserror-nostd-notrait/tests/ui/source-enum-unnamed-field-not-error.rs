use thiserror_nostd_notrait::Error;

#[derive(Debug)]
pub struct NotError;

#[derive(Error, Debug)]
#[error("...")]
pub enum ErrorEnum {
    Broken(#[source] NotError),
}

fn main() {}
