use thiserror_nostd_notrait::Error;

#[derive(Error, Debug)]
pub struct ErrorStruct {
    #[source]
    a: std::io::Error,
    #[source]
    b: anyhow::Error,
}

fn main() {}
