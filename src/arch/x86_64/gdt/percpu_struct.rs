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

use crate::arch::x86_64::gdt::constants::*;
use crate::arch::x86_64::gdt::percpu_stacks::CpuStacks;
use crate::arch::x86_64::gdt::table::Gdt;
use crate::arch::x86_64::gdt::tss::Tss;

// PerCpuGdt owns the GDT and the TSS. The stacks the TSS points at live in
// `percpu_stacks`, because a GDT's non-zero descriptors would otherwise drag
// every stack byte into `.data` with them.
//
// There is one stack for each of the seven IST slots (1..7); a CPU exception
// that uses an IST index must find a non-zero stack pointer in TSS.IST[i] or
// the CPU triple-faults before any handler runs. Whenever a new IDT entry
// calls `set_stack_index(N)`, the matching IST slot in `init` must already be
// set or the static gate over this file fails closed.
#[repr(C, align(64))]
pub struct PerCpuGdt {
    pub gdt: Gdt,
    pub tss: Tss,
    pub cpu_id: u32,
    pub initialized: bool,
}

impl PerCpuGdt {
    pub const fn new() -> Self {
        Self { gdt: Gdt::new(), tss: Tss::new(), cpu_id: 0, initialized: false }
    }

    /// Point this CPU's TSS at `stacks` and publish the TSS in its GDT.
    ///
    /// `stacks` must be the block belonging to this CPU and must outlive it:
    /// the CPU keeps taking faults onto these stacks for as long as it runs.
    pub fn init(&mut self, cpu_id: u32, stacks: &'static CpuStacks) {
        self.cpu_id = cpu_id;
        for slot in
            [IST_DOUBLE_FAULT, IST_NMI, IST_MACHINE_CHECK, IST_DEBUG, IST_PAGE_FAULT, IST_GP]
        {
            let _ = self.tss.set_ist(slot, stacks.ist_top(slot));
        }
        let _ = self.tss.set_ist(IST_RESERVED, stacks.ist_top(IST_RESERVED));
        self.tss.set_rsp0(stacks.kernel_top());
        let tss_addr = &self.tss as *const Tss as u64;
        self.gdt.set_tss(tss_addr);
        self.initialized = true;
    }
}
