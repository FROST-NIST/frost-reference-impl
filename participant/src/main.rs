#[cfg(all(test, not(feature = "ed448")))]
mod tests;

use clap::Parser;
use participant::args::Args;
use participant::cli::cli;

use std::io;

// TODO: Update to use exit codes
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut reader = Box::new(io::stdin().lock());
    let mut logger = io::stdout();
    cli(&args, &mut reader, &mut logger).await?;

    // Force process to exit; since socket comms spawn a thread, it will keep
    // running forever. Ideally we should join() the thread but this works for
    // now.
    std::process::exit(0);
}
