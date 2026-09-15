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

// The Ed25519 subtree references its own field/point/scalar modules by
// absolute `crate::crypto::asymmetric::ed25519::...` paths, so it must live at
// exactly that module path. The `#[path]` include pulls the real source; its
// child modules resolve relative to the real directory.
// `sc_reduce_mod_l` is a `pub(crate)` re-export the kernel consumes elsewhere;
// in this proof crate only sign/verify are exercised, so it reads as unused.
//
// The field/point/scalar arithmetic follows the ref10 reference layout
// (explicit index loops and casts, kept for line-by-line auditability against
// the canonical implementation). Those style lints are allowed on the included
// source rather than rewritten, since the KATs prove behavior and any deviation
// from the reference shape is what crypto review is meant to catch.
#[path = "../../../../../src/crypto/asymmetric/ed25519/mod.rs"]
pub mod ed25519;

// NIST P-256 ECDSA (self-contained; reference-shape field/scalar arithmetic).
#[path = "../../../../../src/crypto/asymmetric/p256/mod.rs"]
pub mod p256;

// NIST P-384 ECDSA (self-contained; reference-shape arithmetic).
#[path = "../../../../../src/crypto/asymmetric/p384/mod.rs"]
pub mod p384;

// secp256k1 is no longer here: the curve moved to userland as nonos_secp256k1,
// which carries its own vectors. It had no in-kernel caller and existed only to
// answer syscalls for the wallet.

// Curve25519 (Edwards point arithmetic) used by the kernel ZK verifier.
// (The module self-allows `needless_range_loop`.)
#[path = "../../../../../src/crypto/asymmetric/curve25519/mod.rs"]
pub mod curve25519;

// RSA (PKCS#1 v1.5 / PSS). Uses the bigint, entropy, error and hash modules
// provided above.
#[path = "../../../../../src/crypto/asymmetric/rsa/mod.rs"]
pub mod rsa;
