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

extern crate alloc;

use alloc::format;
use alloc::string::String;

/// Fields decoded out of `MIDR_EL1`, which is what identifies a core here.
/// There is no CPUID and no brand string to read: an ARM part is named by
/// who implemented it and which part number they gave it, so the entry
/// carries those numbers and lets userspace map them to a name. Linux
/// reports the same set on this architecture, so tools that already parse
/// `/proc/cpuinfo` on ARM see what they expect.
struct Midr {
    implementer: u32,
    variant: u32,
    architecture: u32,
    part: u32,
    revision: u32,
}

impl Midr {
    fn read() -> Self {
        // SAFETY: reading a system register with no memory operand and no
        // side effect. MIDR_EL1 is readable at EL1 on every ARMv8 part.
        let midr: u64 = unsafe {
            let value: u64;
            core::arch::asm!("mrs {0}, midr_el1", out(reg) value, options(nomem, nostack));
            value
        };

        Self {
            implementer: ((midr >> 24) & 0xff) as u32,
            variant: ((midr >> 20) & 0xf) as u32,
            architecture: ((midr >> 16) & 0xf) as u32,
            part: ((midr >> 4) & 0xfff) as u32,
            revision: (midr & 0xf) as u32,
        }
    }
}

pub(super) fn read_cpuinfo() -> String {
    let midr = Midr::read();
    let mut output = String::new();
    for cpu in 0..crate::smp::cpu_count() {
        output.push_str(&format!(
            "processor\t: {}\n\
             CPU implementer\t: {:#04x}\n\
             CPU architecture: {}\n\
             CPU variant\t: {:#x}\n\
             CPU part\t: {:#05x}\n\
             CPU revision\t: {}\n\n",
            cpu, midr.implementer, midr.architecture, midr.variant, midr.part, midr.revision
        ));
    }
    output
}
