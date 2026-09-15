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

//! What the wallet has to know about a blob it never opens.
//!
//! One number, kept in a file with no dependencies so the proofs can include
//! it beside the keyring's own layout and check the two agree. The wallet
//! reads and writes exactly this many bytes and refuses anything else, which
//! is what keeps a truncated file from reaching a TPM derivation.

use nonos_seal::{NONCE_LEN, TAG_LEN};

/// Header 12, nonce 12, secret 32, tag 16.
pub const BLOB_LEN: usize = HEADER_LEN + NONCE_LEN + SECRET_LEN + TAG_LEN;

/// Magic 8, version 2, key type 1, reserved 1.
pub const HEADER_LEN: usize = 12;

pub const SECRET_LEN: usize = 32;
