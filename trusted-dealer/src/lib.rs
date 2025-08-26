pub mod args;
pub mod cli;
pub mod inputs;
pub mod trusted_dealer_keygen;

pub use inputs::Config;

use rand::{CryptoRng, RngCore};
use std::collections::BTreeMap;

use frost_core::keys::{IdentifierList, PublicKeyPackage, SecretShare};
use frost_core::{Ciphersuite, Identifier};

use crate::trusted_dealer_keygen::{split_secret, trusted_dealer_keygen};

#[allow(clippy::type_complexity)]
pub fn trusted_dealer<C: Ciphersuite + 'static, R: RngCore + CryptoRng>(
    config: &Config,
    rng: &mut R,
) -> Result<
    (BTreeMap<Identifier<C>, SecretShare<C>>, PublicKeyPackage<C>),
    Box<dyn std::error::Error>,
> {
    let shares_and_package = if config.secret.is_empty() {
        trusted_dealer_keygen(config, IdentifierList::<C>::Default, rng)?
    } else {
        split_secret(config, IdentifierList::<C>::Default, rng)?
    };

    let (shares, pubkeys) = shares_and_package;

    Ok((shares, pubkeys))
}
