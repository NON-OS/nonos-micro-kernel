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

#![allow(clippy::many_single_char_names)]
#![allow(clippy::identity_op)]

extern crate alloc;

mod api;
mod chunk;
mod compress;
mod constants;
mod hasher;
mod output;

pub use api::{blake3_derive_key, blake3_hash, blake3_hash_xof, blake3_keyed_hash};
pub use constants::{KEY_LEN, OUT_LEN};
pub use hasher::Hasher;
pub use output::OutputReader;

pub(crate) use constants::{
    BLOCK_LEN, CHUNK_END, CHUNK_LEN, CHUNK_START, DERIVE_KEY_CONTEXT, DERIVE_KEY_MATERIAL, IV,
    KEYED_HASH, MAX_DEPTH, MSG_SCHEDULE, PARENT, ROOT, ROUNDS,
};
