use thiserror_nostd_notrait::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("...")]
    A(usize),
    B(usize),
}

fn main() {}
