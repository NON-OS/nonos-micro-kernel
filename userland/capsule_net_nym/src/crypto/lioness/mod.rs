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

//! LIONESS, the wide-block cipher Sphinx encrypts payloads with.
//!
//! A payload is one block however long it is, so a single changed ciphertext
//! bit scrambles the whole plaintext. That is what stops a mix from tagging a
//! packet by flipping a payload bit and recognising it downstream.

mod decrypt_block;
mod encrypt_block;
mod mac_into_left;
mod new;
mod stream_into_right;
mod types;
mod wipe;

pub use types::{BlockTooShort, Lioness, KEY_BYTES, MAC_BYTES, MAC_KEY_BYTES};
