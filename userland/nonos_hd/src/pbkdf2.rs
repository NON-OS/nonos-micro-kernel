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

// RFC 2898 PBKDF2 with HMAC-SHA512, fixed to the 64-byte output BIP39 needs,
// which is exactly one PRF block: F(1) = U1 xor U2 xor ... xor Uc with
// U1 = PRF(password, salt || 0x00000001). Allocation-free; every
// intermediate U value is wiped before return.

use crate::hmac512::{hmac_sha512, HmacSha512};
use crate::wipe::wipe;

pub fn pbkdf2_hmac_sha512(password: &[u8], salt: &[u8], iterations: u32, out: &mut [u8; 64]) {
    let mut mac = HmacSha512::new(password);
    mac.update(salt);
    mac.update(&1u32.to_be_bytes());
    let mut u = mac.finalize();

    out.copy_from_slice(&u);
    for _ in 1..iterations {
        let next = hmac_sha512(password, &u);
        wipe(&mut u);
        u = next;
        for (o, n) in out.iter_mut().zip(u.iter()) {
            *o ^= n;
        }
    }
    wipe(&mut u);
}
