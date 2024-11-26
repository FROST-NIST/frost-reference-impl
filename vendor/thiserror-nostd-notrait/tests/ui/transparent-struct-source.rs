use thiserror_nostd_notrait::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(#[source] anyhow::Error);

fn main() {}
