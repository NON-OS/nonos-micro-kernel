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

// Mirrors `crate::crypto::constant_time` from the kernel (`pub use
// util::constant_time`) so the included HMAC source resolves its dependency
// against the real constant-time primitives, not a copy.
//
// The constant-time primitives deliberately avoid the forms clippy considers
// idiomatic: per-index copies instead of `copy_from_slice`, full-table index
// scans instead of iterators, hand-written rotates. Those manual shapes are
// the point (fixed, data-independent timing), so the style lints are allowed
// on the included source rather than "fixed" into timing side channels.
#[allow(clippy::manual_memcpy, clippy::needless_range_loop, clippy::manual_rotate)]
#[path = "../../../../src/crypto/util/constant_time/mod.rs"]
pub mod constant_time;

// Ed25519 (the trust-root signature primitive) and its dependencies. The
// subtree is self-contained apart from SHA-512 and the RNG, mirrored here.
pub mod asymmetric;
pub mod rng;
pub mod sha512;

// ChaCha20-Poly1305 AEAD; depends only on the constant-time primitives above.
pub mod symmetric;

// P-256's RFC 6979 deterministic nonce resolves `crate::crypto::hmac_sha256`,
// and `verify_message` resolves `crate::crypto::hash::*`.
pub use crate::hash;
pub use crate::hash::hmac_sha256;
// secp256k1's Ethereum-address helper resolves `crate::crypto::sha3`.
pub use crate::hash::sha3;

// secp256k1 returns `crate::crypto::{CryptoError, CryptoResult}`.
#[path = "../../../../src/crypto/error.rs"]
pub mod error;
pub use error::{CryptoError, CryptoResult};

// Big integers (RSA) and an entropy shim (RSA/ECDSA keygen, unused by KATs).
pub mod entropy;
pub mod util;
