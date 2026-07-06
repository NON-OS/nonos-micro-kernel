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

// Mirrors the kernel's `crypto::hash::unified` parent so the included hash
// source resolves `super::Hash256`.
pub type Hash256 = [u8; 32];

#[path = "../../../../src/crypto/hash/unified/sha256.rs"]
pub mod sha256;
#[path = "../../../../src/crypto/hash/sha512/mod.rs"]
pub mod sha512;

// Mirror the kernel's `unified` module: expose the `sha256` function (not just
// the module) so the included HMAC source resolves `use super::sha256`.
pub use sha256::sha256;

#[path = "../../../../src/crypto/hash/unified/hmac.rs"]
pub mod hmac;

// Expose the HMAC functions at the parent level so the included HKDF source
// resolves `use super::hmac_sha256`.
pub use hmac::{hmac_sha256, hmac_verify};

#[path = "../../../../src/crypto/hash/unified/hkdf.rs"]
pub mod hkdf;

// SHA-3 / Keccak family (self-contained under its own module).
#[allow(
    clippy::needless_range_loop,
    clippy::manual_rotate,
    clippy::identity_op,
    clippy::unnecessary_cast,
    clippy::manual_is_multiple_of
)]
#[path = "../../../../src/crypto/hash/sha3/mod.rs"]
pub mod sha3;
