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

use core::arch::asm;

pub fn send_sgi(target: u32, intid: u32) -> Result<(), ()> {
    if intid >= 16 {
        return Err(());
    }
    let aff = target_to_affinity(target);
    let value = ((intid as u64) << 24) | (aff & 0xFFFF) | ((aff & 0xFF0000) << 16);
    unsafe {
        asm!("msr icc_sgi1r_el1, {0}", "isb", in(reg) value, options(nostack));
    }
    Ok(())
}

fn target_to_affinity(target: u32) -> u64 {
    let target_list = 1u64 << (target & 0xF);
    let aff1 = ((target >> 4) & 0xFF) as u64;
    let aff2 = ((target >> 12) & 0xFF) as u64;
    let aff3 = ((target >> 20) & 0xFF) as u64;
    target_list | (aff1 << 16) | (aff2 << 32) | (aff3 << 48)
}
