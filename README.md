# tss2-ts

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![crates.io](https://img.shields.io/crates/v/tss2.svg)](https://crates.io/crates/tpm2-wolf)

Implements raw TPM2 Rust wrapper around wolfTPM libraries.

### Usage

Install wolfSSL and wolfTPM libs to the system.

Add this to your `Cargo.toml`:

```toml
[dependencies]
tpm2-wolf = "0"
```

Import necessary types and functions and use as you would do with native wolfTPM.
Do not forget to free memory allocated inside C library.