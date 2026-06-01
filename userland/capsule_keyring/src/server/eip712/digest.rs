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

use super::consts::DOMAIN_SEPARATOR;

pub fn receipt_digest(struct_hash: &[u8; 32]) -> Option<[u8; 32]> {
    let mut pre = [0u8; 66];
    pre[0] = 0x19;
    pre[1] = 0x01;
    pre[2..34].copy_from_slice(&DOMAIN_SEPARATOR);
    pre[34..66].copy_from_slice(struct_hash);
    let mut out = [0u8; 32];
    if nonos_libc::crypto_keccak256(pre.as_ptr(), 66, out.as_mut_ptr(), 32) != 32 {
        return None;
    }
    Some(out)
}
