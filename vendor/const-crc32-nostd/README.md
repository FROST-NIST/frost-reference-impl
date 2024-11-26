# const-crc32

A `const fn` crc32 checksum implementation.

## Examples

```rust
const BYTES: &[u8] = "The quick brown fox jumps over the lazy dog".as_bytes();
const CKSUM: u32 = const_crc32::crc32(BYTES);
assert_eq!(CKSUM, 0x414fa339_u32);
```

## Usage

This is a naive implementation that should be expected to have poor performance
if used on dynamic data at runtime. Usage should generally be restricted to declaring
`const` variables based on `static` or `const` data available at build time.

## `#[const_eval_limit]`

You may need to increase the crate-wide `const_eval_limit` setting to use `const_crc32` for larger byte slices.

Increating `const_eval_limit` requires the nightly-only `#![feature(const_eval_limit)]`.

Previously, this crate set the limit itself, however, as of the 2022-10-30 nightly, the value set in `const_crc32` does not increase the limit for crates which use the library.

Compile time for `const` data around 100k is less than 1s.
