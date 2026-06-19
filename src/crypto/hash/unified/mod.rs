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

mod blake3;
mod hkdf;
pub(crate) mod hkdf_sha384;
mod hmac;
pub(crate) mod hmac_sha384;
mod ripemd160;
pub mod sha256;
mod sha3;
// SHA-1 needed for WPA compatibility
pub mod sha1;

pub type Hash256 = [u8; 32];

pub use blake3::blake3_hash;
pub use hkdf::{hkdf_expand, hkdf_extract};
pub use hkdf_sha384::{hkdf_expand_sha384, hkdf_extract_sha384};
pub use hmac::{hmac_sha256, hmac_verify};
pub use hmac_sha384::{hmac_sha384, hmac_sha384_verify};
pub use ripemd160::ripemd160;
pub use sha256::sha256;
pub use sha3::sha3_256_hash;

// SHA-1 for WPA compatibility
#[allow(deprecated)]
pub use sha1::sha1;
