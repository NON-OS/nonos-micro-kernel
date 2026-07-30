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

//! Which address this station transmits from.
//!
//! The EEPROM address is unique to one card and is the first thing every
//! access point on the way records, so a system that keeps nothing on disk
//! still arrives everywhere carrying the same name. This draws a fresh
//! locally administered address instead, once per bring-up, which on a
//! RAM-resident system means once per boot.

use nonos_mac::{from_random, MAC_LEN};

/// Draw a station address from kernel entropy.
///
/// Fails closed. A driver that cannot get randomness must not quietly fall
/// back to the EEPROM address, because the fallback is the exact identifier
/// this exists to stop transmitting, and it would take the one path nobody
/// looks at.
pub fn draw() -> Result<[u8; MAC_LEN], &'static str> {
    let mut bytes = [0u8; MAC_LEN];
    let rc = unsafe { nonos_libc::crypto_random(bytes.as_mut_ptr(), bytes.len()) };
    if rc < 0 || (rc as usize) != bytes.len() {
        return Err("no entropy for station address");
    }
    Ok(from_random(bytes))
}
