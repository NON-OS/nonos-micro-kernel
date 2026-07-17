// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The Goldilocks prime field, p = 2^64 - 2^32 + 1. It is 64-bit friendly, has
//! a large two-adic subgroup for the FFT a prover needs, and uses only integer
//! arithmetic, so verification is hash-and-integer only and post-quantum.

mod element;
mod exp;
mod ext;
mod felt;
mod ops;

pub use element::{Fp, P};
pub use ext::Fp2;
pub use felt::Felt;
