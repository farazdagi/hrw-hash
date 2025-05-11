# hrw-hash

[![crates.io](https://img.shields.io/crates/d/hrw-hash.svg)](https://crates.io/crates/hrw-hash)
[![docs.rs](https://docs.rs/hrw-hash/badge.svg)](https://docs.rs/hrw-hash)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![dependencies](https://deps.rs/repo/github/farazdagi/hrw-hash/status.svg)](https://deps.rs/repo/github/farazdagi/mpchash)

Minimalistic implementation of the Highest Random Weight (HRW) aka Rendezvous hashing algorithm as
described in the
["A name-based mapping scheme for Rendezvous"](https://www.eecs.umich.edu/techreports/cse/96/CSE-TR-316-96.pdf),
by Thaler and Ravishankar (1996).

The weighted variant of the HRW algorithm is implemented using Logarithmic Method as described in
["Weighted distributed hash tables"](https://dl.acm.org/doi/10.1145/1073970.1074008), by
Schindelhauer and Schomaker (2005).

## Features

- [ ] Absolutely minimalistic implementation with sane defaults.
- [ ] Allow weighted nodes.

## Usage

The core functionality is exposed via `HrwHash` trait:

``` rust

```


## License

MIT
