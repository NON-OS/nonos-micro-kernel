// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg(feature = "zk-halo2")]

extern crate alloc;

mod api;
mod deserialize;
pub mod params;
mod verifier;

use alloc::vec::Vec;
use core::fmt;

pub use api::{halo2_verify, halo2_verify_with_format};
pub use halo2_proofs::SerdeFormat;
pub use verifier::Halo2Verifier;

pub(crate) const MAX_PARAMS_BYTES: usize = 64 * 1024 * 1024;
pub(crate) const MAX_VK_BYTES: usize = 16 * 1024 * 1024;
pub(crate) const MAX_PROOF_BYTES: usize = 1 * 1024 * 1024;
pub(crate) const MAX_PUBLIC_INPUTS: usize = 1 << 20;
pub(crate) const MIN_K: u32 = 4;
pub(crate) const MAX_K: u32 = 24;

pub(crate) const FR_MODULUS_BYTES: [u8; 32] = [
    0x01, 0x00, 0x00, 0xf0, 0x93, 0xf5, 0xe1, 0x43, 0x91, 0x70, 0xb9, 0x79, 0x48, 0xe8, 0x33, 0x28,
    0x5d, 0x58, 0x81, 0x81, 0xb6, 0x45, 0x50, 0xb8, 0x29, 0xa0, 0x31, 0xe1, 0x72, 0x4e, 0x64, 0x30,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Halo2Error {
    Deserialize(&'static str),
    SizeLimit(&'static str),
    PublicInputShape,
    KOutOfRange,
    VerifyFailed,
    InvalidFieldElement,
    IoError,
}

impl fmt::Display for Halo2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Halo2Error::Deserialize(m) => write!(f, "deserialize error: {}", m),
            Halo2Error::SizeLimit(m) => write!(f, "size exceeds limit: {}", m),
            Halo2Error::PublicInputShape => write!(f, "public input shape mismatch"),
            Halo2Error::KOutOfRange => {
                write!(f, "circuit size k outside range [{}, {}]", MIN_K, MAX_K)
            }
            Halo2Error::VerifyFailed => write!(f, "proof verification failed"),
            Halo2Error::InvalidFieldElement => write!(f, "field element out of range"),
            Halo2Error::IoError => write!(f, "I/O error"),
        }
    }
}
