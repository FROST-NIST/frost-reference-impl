use std::{
    error::Error,
    fs,
    io::{BufRead, Write},
};

use clap::Parser;

use frost_core::{keys::PublicKeyPackage, Ciphersuite};

use crate::input::read_from_file_or_stdin;

#[derive(Clone, Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'C', long, default_value = "ed25519")]
    pub ciphersuite: String,

    /// CLI mode. If enabled, it will prompt for inputs from stdin
    /// and print values to stdout, ignoring other flags.
    /// If false, socket communication is enabled.
    #[arg(long, default_value_t = false)]
    pub cli: bool,

    /// The number of participants. If `signers` is specified, it will use the
    /// length of `signers`. Otherwise, if 0, it will prompt for a value.
    #[arg(short = 'n', long, default_value_t = 0)]
    pub num_signers: u16,

    /// Public key package to use. Can be a file with a JSON-encoded
    /// package, or "-". If the file does not exist or if "-" is specified,
    /// then it will be read from standard input.
    #[arg(short = 'P', long, default_value = "public-key-package.json")]
    pub public_key_package: String,

    /// The messages to sign. Each instance can be a file with the raw message,
    /// "" or "-". If "" or "-" is specified, then it will be read from standard
    /// input as a hex string. If none are passed, a single one will be read
    /// from standard input as a hex string.
    #[arg(short = 'm', long)]
    pub message: Vec<String>,

    /// Where to write the generated raw bytes signature. If "-", the
    /// human-readable hex-string is printed to stdout.
    #[arg(short = 's', long, default_value = "")]
    pub signature: String,

    /// IP to bind to, if using socket comms.
    /// IP to connect to, if using HTTP mode.
    #[arg(short, long, default_value = "0.0.0.0")]
    pub ip: String,

    /// Port to bind to, if using socket comms.
    /// Port to connect to, if using HTTP mode.
    #[arg(short, long, default_value_t = 2744)]
    pub port: u16,
}

#[derive(Clone)]
pub struct ProcessedArgs<C: Ciphersuite> {
    /// CLI mode. If enabled, it will prompt for inputs from stdin
    /// and print values to stdout, ignoring other flags.
    /// If false, socket communication is enabled.
    pub cli: bool,

    /// The number of participants.
    pub num_signers: u16,

    /// Public key package to use.
    pub public_key_package: PublicKeyPackage<C>,

    /// The messages to sign.
    pub messages: Vec<Vec<u8>>,

    /// Where to write the generated raw bytes signature. If "-", the
    /// human-readable hex-string is printed to stdout.
    pub signature: String,

    /// IP to bind to, if using socket comms.
    /// IP to connect to, if using HTTP mode.
    pub ip: String,

    /// Port to bind to, if using socket comms.
    /// Port to connect to, if using HTTP mode.
    pub port: u16,
}

impl<C: Ciphersuite + 'static> ProcessedArgs<C> {
    /// Create a ProcessedArgs from a Args.
    ///
    /// Validates inputs and reads/parses arguments.
    pub fn new(
        args: &Args,
        input: &mut dyn BufRead,
        output: &mut dyn Write,
    ) -> Result<Self, Box<dyn Error>> {
        let num_signers = if args.num_signers == 0 {
            writeln!(output, "The number of participants: ")?;

            let mut participants = String::new();
            input.read_line(&mut participants)?;
            participants.trim().parse::<u16>()?
        } else {
            args.num_signers
        };

        let out = read_from_file_or_stdin(
            input,
            output,
            "public key package",
            &args.public_key_package,
        )?;

        let public_key_package: PublicKeyPackage<C> = serde_json::from_str(&out)?;

        let messages = read_messages(&args.message, output, input)?;

        Ok(ProcessedArgs {
            cli: args.cli,
            num_signers,
            public_key_package,
            messages,
            signature: args.signature.clone(),
            ip: args.ip.clone(),
            port: args.port,
        })
    }
}

pub fn read_messages(
    message_paths: &[String],
    output: &mut dyn Write,
    input: &mut dyn BufRead,
) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let messages = if message_paths.is_empty() {
        writeln!(output, "The message to be signed (hex encoded)")?;
        let mut msg = String::new();
        input.read_line(&mut msg)?;
        vec![hex::decode(msg.trim())?]
    } else {
        message_paths
            .iter()
            .map(|filename| {
                let msg = if *filename == "-" || filename.is_empty() {
                    writeln!(output, "The message to be signed (hex encoded)")?;
                    let mut msg = String::new();
                    input.read_line(&mut msg)?;
                    hex::decode(msg.trim())?
                } else {
                    eprintln!("Reading message from {}...", &filename);
                    fs::read(filename)?
                };
                Ok(msg)
            })
            .collect::<Result<_, Box<dyn Error>>>()?
    };
    Ok(messages)
}
