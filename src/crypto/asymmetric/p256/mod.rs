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

mod constants;
pub mod ecdh;
mod ecdsa;
mod field;
mod point;
mod scalar;

pub use constants::{CompressedPublicKey, PublicKey, SecretKey, Signature};

pub use ecdh::{p256_ecdh, p256_ecdh_keypair};

pub use ecdsa::{
    generate_keypair, public_key_from_secret, sign, sign_message, verify, verify_message,
    P256KeyPair,
};

pub use field::FieldElement;

pub use point::{AffinePoint, ProjectivePoint};

pub use scalar::Scalar;

pub(crate) use constants::{P256_A, P256_B, P256_GX, P256_GY, P256_N, P256_P};
