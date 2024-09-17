# FROST Reference Implementation

This package contain the FROST reference implementation written in Rust.

The implementation is contained in different crates, as follows:

- `frost-core`: a generic implementation of FROST using a `Ciphersuite` trait.
- `frost-ed25519`: Ed25519-compatible FROST `Ciphersuite` implementation.
  Also re-exports the `frost-core` API without generics.
- `frost-ed448`: Ed448-compatible FROST `Ciphersuite` implementation.
  Also re-exports the `frost-core` API without generics.

Some additional crates are provided. They simply serve as examples on how to use
the above crates, and are not provided to be used directly by applications or
users.

- `trusted-dealer`: a command-line tool that generates FROST shares using
  trusted dealer key generation.
- `dkg`: a command-line tool that generates FROST shares using distributed key
  generation.
- `coordinator`: a command-line tool that executes a FROST protocol signing
  round as the coordinator, opening a socket to communicate with participants.
- `participant`: a command-line tool that executes a FROST protocol signing
  round as a participant, connecting to the `coordinator` tool.
