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

//! Roots that belong in the pinned set but were missed when the four chunks
//! were cut. The subject index in `store.bin` carried them, so a chain that
//! reached one through a cross-sign still anchored, but a chain that ended on
//! the served root itself did not, because the final `is_trusted_spki_hash`
//! never matched. Keeping them here rather than resizing a chunk leaves the
//! generated tables untouched and makes the addition and its reason obvious.

/// ISRG Root X2, Let's Encrypt's ECDSA (P-384) root. Chains from validator
/// endpoints and much of the modern web terminate here directly, not only
/// through the X1 cross-sign, so its SPKI hash has to be trusted on its own.
/// SHA-256 of the SubjectPublicKeyInfo: 762195c2…cce68332.
pub const EXTRA_ROOTS: [[u8; 32]; 1] = [[
    118, 33, 149, 194, 37, 88, 110, 230, 192, 35, 116, 86, 226, 16, 125, 197, 79, 30, 252, 33,
    246, 26, 121, 46, 189, 81, 89, 19, 204, 230, 131, 50,
]];
