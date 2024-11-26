pub mod cli;
pub mod socket;

use async_trait::async_trait;

use frost_core::{self as frost, Ciphersuite};

use std::{
    error::Error,
    io::{BufRead, Write},
};

use frost::{
    round1::SigningCommitments,
    round2::SignatureShare,
    serde::{self, Deserialize, Serialize},
    Identifier,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "self::serde")]
#[serde(bound = "C: Ciphersuite")]
#[allow(clippy::large_enum_variant)]
pub enum Message<C: Ciphersuite> {
    IdentifiedCommitments {
        identifier: Identifier<C>,
        commitments: SigningCommitments<C>,
    },
    SigningPackage {
        signing_package: frost::SigningPackage<C>,
    },
    SignatureShare(SignatureShare<C>),
}

#[async_trait(?Send)]
pub trait Comms<C: Ciphersuite> {
    async fn get_signing_package(
        &mut self,
        input: &mut dyn BufRead,
        output: &mut dyn Write,
        commitments: SigningCommitments<C>,
        identifier: Identifier<C>,
    ) -> Result<frost::SigningPackage<C>, Box<dyn Error>>;

    async fn send_signature_share(
        &mut self,
        identifier: Identifier<C>,
        signature_share: SignatureShare<C>,
    ) -> Result<(), Box<dyn Error>>;
}
