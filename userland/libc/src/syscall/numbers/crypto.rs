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
use super::tag::tag4;

pub(crate) const N_CRYPTO_RANDOM: i64 = tag4(b"CRND");
pub(crate) const N_CRYPTO_HASH: i64 = tag4(b"CHSH");
pub(crate) const N_CRYPTO_ENCRYPT: i64 = tag4(b"CENC");
pub(crate) const N_CRYPTO_DECRYPT: i64 = tag4(b"CDEC");
pub(crate) const N_CRYPTO_ENCRYPT_AAD: i64 = tag4(b"CEAD");
pub(crate) const N_CRYPTO_DECRYPT_AAD: i64 = tag4(b"CDAD");
pub(crate) const N_CRYPTO_ED25519_VERIFY: i64 = tag4(b"CEDV");
pub(crate) const N_CRYPTO_ED25519_SIGN: i64 = tag4(b"CEDS");
pub(crate) const N_CRYPTO_ED25519_PUBKEY: i64 = tag4(b"CEDP");
pub(crate) const N_CRYPTO_X25519_PUBLIC: i64 = tag4(b"CXPK");
pub(crate) const N_CRYPTO_X25519_SHARED: i64 = tag4(b"CXSH");
pub(crate) const N_CRYPTO_HMAC_SHA256: i64 = tag4(b"CHMC");
pub(crate) const N_CRYPTO_HKDF_SHA256: i64 = tag4(b"CHKF");
pub(crate) const N_CRYPTO_KECCAK256: i64 = tag4(b"CKEC");
pub(crate) const N_CRYPTO_SECP256K1_SIGN: i64 = tag4(b"CSKS");
pub(crate) const N_CRYPTO_SECP256K1_PUBKEY: i64 = tag4(b"CSPB");
