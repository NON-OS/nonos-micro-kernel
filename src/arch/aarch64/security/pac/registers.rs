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

use super::key::PacKeys;

pub fn install_keys(keys: &PacKeys) {
    unsafe {
        asm!("msr apiakeylo_el1, {}", in(reg) keys.ia.lo);
        asm!("msr apiakeyhi_el1, {}", in(reg) keys.ia.hi);
        asm!("msr apibkeylo_el1, {}", in(reg) keys.ib.lo);
        asm!("msr apibkeyhi_el1, {}", in(reg) keys.ib.hi);
        asm!("msr apdakeylo_el1, {}", in(reg) keys.da.lo);
        asm!("msr apdakeyhi_el1, {}", in(reg) keys.da.hi);
        asm!("msr apdbkeylo_el1, {}", in(reg) keys.db.lo);
        asm!("msr apdbkeyhi_el1, {}", in(reg) keys.db.hi);
        asm!("msr apgakeylo_el1, {}", in(reg) keys.ga.lo);
        asm!("msr apgakeyhi_el1, {}", in(reg) keys.ga.hi);
        asm!("isb");
    }
}
