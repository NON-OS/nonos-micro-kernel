// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

pub const FOOTER_MAGIC: [u8; 8] = *b"NONOSIMG";
pub const FOOTER_VERSION: u16 = 1;
pub const FOOTER_SIZE: usize = 64;
pub const HASH_ALG_BLAKE3: u8 = 1;
pub const SIG_ALG_ED25519_MLDSA65: u8 = 2;
pub const ED25519_SIG_SIZE: usize = 64;
pub const MLDSA65_SIG_SIZE: usize = 3309;
pub const RELEASE_SIG_MAGIC: [u8; 8] = *b"NKRSIG2\0";
pub const RELEASE_SIG_SIZE: usize = 8 + 32 + ED25519_SIG_SIZE + 32 + MLDSA65_SIG_SIZE;
