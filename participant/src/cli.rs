use crate::args::{Args, ProcessedArgs};

use crate::comms::cli::CLIComms;
use crate::comms::socket::SocketComms;

use crate::comms::Comms;

use crate::round1::{generate_nonces_and_commitments, print_values};
use crate::round2::{generate_signature, print_values_round_2, round_2_request_inputs};
use frost_core::Ciphersuite;
use rand::thread_rng;
use std::io::{BufRead, Write};

pub async fn cli<C: Ciphersuite + 'static>(
    args: &Args,
    reader: &mut impl BufRead,
    logger: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let pargs = ProcessedArgs::<C>::new(args, reader, logger)?;
    cli_for_processed_args(pargs, reader, logger).await
}

pub async fn cli_for_processed_args<C: Ciphersuite + 'static>(
    pargs: ProcessedArgs<C>,
    input: &mut impl BufRead,
    logger: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut comms: Box<dyn Comms<C>> = if pargs.cli {
        Box::new(CLIComms::new())
    } else {
        Box::new(SocketComms::new(&pargs))
    };

    // Round 1

    let key_package = pargs.key_package;

    writeln!(logger, "Key Package succesfully created.")?;

    let mut rng = thread_rng();
    let (nonces, commitments) = generate_nonces_and_commitments(&key_package, &mut rng);

    print_values(commitments, logger)?;

    // Round 2 - Sign

    let round_2_config = round_2_request_inputs(
        &mut *comms,
        input,
        logger,
        commitments,
        *key_package.identifier(),
    )
    .await?;
    let signature = generate_signature(round_2_config, &key_package, &nonces)?;

    comms
        .send_signature_share(*key_package.identifier(), signature)
        .await?;

    print_values_round_2(signature, logger)?;

    Ok(())
}
