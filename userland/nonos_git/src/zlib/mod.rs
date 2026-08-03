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

//! zlib (RFC 1950) around DEFLATE (RFC 1951), the wrapping every git object
//! wears on disk.
//!
//! `decompress` is a full inflate over stored, fixed-Huffman and
//! dynamic-Huffman blocks, so it reads objects git itself wrote. `compress`
//! writes stored blocks: a valid, git-readable stream with a small auditable
//! writer, since the objects a terminal session creates are small and the read
//! path is the one that must handle anything. Both ends carry the Adler-32 the
//! format checks.

mod adler;
mod bit_reader;
mod compress;
mod dynamic;
mod error;
mod huffman;
mod huffman_block;
mod inflate;
mod prefix;
mod stored;
mod tables;

pub use compress::compress;
pub use error::InflateError;
pub use inflate::decompress;
pub use prefix::decompress_prefix;
