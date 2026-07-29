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

//! SHA-384: same compression as SHA-512, different IV, output truncated to 48 bytes.

use super::sha512::Sha512;

/// SHA-384 initial hash values (FIPS 180-4 §5.3.4).
const SHA384_IV: [u64; 8] = [
    0xcbbb9d5dc1059ed8,
    0x629a292a367cd507,
    0x9159015a3070dd17,
    0x152fecd8f70e5939,
    0x67332667ffc00b31,
    0x8eb44a8768581511,
    0xdb0c2e0d64f98fa7,
    0x47b5481dbefa4fa4,
];

pub type Hash384 = [u8; 48];

pub fn sha384(data: &[u8]) -> Hash384 {
    // SHA-384 reuses the SHA-512 engine but with different initial state
    // and truncates the output to 384 bits (48 bytes).
    let mut hasher = Sha512::new();
    // Override the initial state to SHA-384 IV
    hasher.set_state(SHA384_IV);
    hasher.update(data);
    let full = hasher.finalize();
    let mut out = [0u8; 48];
    out.copy_from_slice(&full[..48]);
    out
}
