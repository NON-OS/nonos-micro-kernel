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

use nonos_mac::apply;

use crate::constants::regs::{CFG9346_LOCK, CFG9346_UNLOCK, REG_CFG9346, REG_MAC0};
use crate::constants::MAC_LEN;
use crate::regs::Regs;

/// Draw a station address and program it into the IDR registers.
///
/// Replaces reading the factory address out of them. Fails closed: the factory
/// address is the identifier this avoids, so it is not a fallback.
pub fn program(regs: &Regs) -> Result<[u8; MAC_LEN], &'static str> {
    let mut mac = [0u8; MAC_LEN];
    let rc = nonos_libc::crypto_random(mac.as_mut_ptr(), MAC_LEN);
    if rc < 0 || (rc as usize) != MAC_LEN {
        return Err("rtl8169 no entropy for station address");
    }
    apply(&mut mac);

    // IDR writes are dropped while the config lock is set.
    unsafe {
        regs.w8(REG_CFG9346, CFG9346_UNLOCK);
        for (i, byte) in mac.iter().enumerate() {
            regs.w8(REG_MAC0 + i, *byte);
        }
        regs.w8(REG_CFG9346, CFG9346_LOCK);
    }

    let mut readback = [0u8; MAC_LEN];
    for (i, byte) in readback.iter_mut().enumerate() {
        *byte = unsafe { regs.r8(REG_MAC0 + i) };
    }
    if readback != mac {
        return Err("rtl8169 station address did not take");
    }
    Ok(mac)
}
