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
//! The hash primitives shared by the wallet, the signing crates and the key
//! derivation. Moved here from nonos_hd so a crate that needs SHA-512 does
//! not carry BIP39 and a word list to get it.

#![no_std]

mod hmac512;
mod sha256;
mod sha512;
mod wipe;

pub use hmac512::{hmac_sha512, HmacSha512};
pub use sha256::sha256;
pub use sha512::{sha512, Sha512};
pub use wipe::wipe;
