#![deny(deprecated, clippy::all, clippy::pedantic)]

use thiserror_nostd_notrait::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[deprecated]
    #[error("...")]
    Deprecated,
}
