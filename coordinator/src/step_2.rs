#[cfg(not(feature = "ed448"))]
use frost_ed25519 as frost;
#[cfg(feature = "ed448")]
use frost_ed448 as frost;

use frost::{round1::SigningCommitments, Identifier, SigningPackage};

use std::{
    collections::BTreeMap,
    fs,
    io::{BufRead, Write},
};

use crate::args::Args;

#[derive(Debug, PartialEq, Clone)]
pub struct CommitmentsConfig {
    pub message: Vec<u8>,
    pub signer_commitments: BTreeMap<Identifier, SigningCommitments>,
}

pub async fn step_2(
    args: &Args,
    input: &mut impl BufRead,
    logger: &mut dyn Write,
    commitments: BTreeMap<Identifier, SigningCommitments>,
) -> Result<SigningPackage, Box<dyn std::error::Error>> {
    let signing_package = request_message(args, input, logger, commitments)?;
    print_signing_package(logger, &signing_package);
    Ok(signing_package)
}

// Input required:
// 1. message
fn request_message(
    args: &Args,
    input: &mut impl BufRead,
    logger: &mut dyn Write,
    commitments: BTreeMap<Identifier, SigningCommitments>,
) -> Result<SigningPackage, Box<dyn std::error::Error>> {
    let message = if args.cli && args.message == "-" {
        writeln!(logger, "The message to be signed (hex encoded)")?;

        let mut msg = String::new();
        input.read_line(&mut msg)?;

        hex::decode(msg.trim())?
    } else {
        fs::read(&args.message)?
    };

    let signing_package = SigningPackage::new(commitments, &message);

    Ok(signing_package)
}

fn print_signing_package(logger: &mut dyn Write, signing_package: &SigningPackage) {
    writeln!(
        logger,
        "Signing Package:\n{}",
        serde_json::to_string(&signing_package).unwrap()
    )
    .unwrap();
}
