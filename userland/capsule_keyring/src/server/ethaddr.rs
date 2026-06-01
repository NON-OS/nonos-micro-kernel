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

pub fn address_of(secret: &[u8; 32]) -> Option<[u8; 20]> {
    let mut pubkey = [0u8; 65];
    if nonos_libc::crypto_secp256k1_pubkey(secret.as_ptr(), pubkey.as_mut_ptr()) != 65 {
        return None;
    }
    let mut hash = [0u8; 32];
    if nonos_libc::crypto_keccak256(pubkey[1..].as_ptr(), 64, hash.as_mut_ptr(), 32) != 32 {
        return None;
    }
    let mut addr = [0u8; 20];
    addr.copy_from_slice(&hash[12..32]);
    Some(addr)
}
