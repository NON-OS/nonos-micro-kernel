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

use super::super::blake2b::Blake2b;
use super::types::{MAC_BYTES, MAC_KEY_BYTES};
use super::wipe::wipe;

pub(super) fn mac_into_left(left: &mut [u8], right: &[u8], key: &[u8; MAC_KEY_BYTES]) {
    let mut mac = Blake2b::new_keyed(key, MAC_BYTES);
    mac.update(right);
    let mut digest = [0u8; MAC_BYTES];
    mac.finalize(&mut digest);
    for (l, d) in left.iter_mut().zip(digest.iter()) {
        *l ^= *d;
    }
    wipe(&mut digest);
}
