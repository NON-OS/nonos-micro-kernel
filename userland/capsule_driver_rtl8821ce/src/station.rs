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

//! The address this radio transmits under. Every AP logs the source of a probe
//! request whether it associates or not, so the efuse address is not used.

use nonos_mac::{apply, MAC_LEN};

/// Draw a station address. `None` is fatal: falling back to the efuse would
/// transmit the identifier this avoids.
pub fn draw() -> Option<[u8; MAC_LEN]> {
    let mut mac = [0u8; MAC_LEN];
    let rc = nonos_libc::crypto_random(mac.as_mut_ptr(), MAC_LEN);
    if rc < 0 || (rc as usize) != MAC_LEN {
        return None;
    }
    apply(&mut mac);
    Some(mac)
}
