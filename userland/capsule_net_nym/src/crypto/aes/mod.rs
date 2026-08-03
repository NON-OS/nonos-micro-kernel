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

//! AES-128 and the counter mode Sphinx builds its header keystream with.
//!
//! This lives in the capsule, not behind a syscall. A raw stream cipher handed
//! to every capsule is a footgun: reuse a key and IV pair once and the XOR of
//! two plaintexts falls out. Sphinx derives a fresh key per hop, so the
//! constraint holds here and the kernel does not grow to carry it.

mod add_round_key;
mod aes256;
mod ctr;
mod encrypt_block;
mod key_schedule;
mod mix_columns;
mod sbox;
mod shift_rows;
mod sub_bytes;
mod sub_word;
mod types;
mod xtime;

pub use aes256::{Aes256, KEY_BYTES as KEY_BYTES_256};
pub use ctr::Ctr64Be;
pub use types::{Aes128, BLOCK_BYTES, KEY_BYTES};
