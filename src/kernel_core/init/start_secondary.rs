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

pub(super) fn start_secondary_cpus() {
    // The IPI path reaches the LAPIC through the arch module's state; adopt
    // the live mode and base from the boot-time init before the first ICR
    // write, or the SIPI dereferences a null register base.
    #[cfg(feature = "nonos-smp")]
    if crate::arch::x86_64::interrupt::apic::adopt_bsp_state().is_err() {
        crate::sys::serial::println(b"[SMP-PROOF] FAIL lapic state not adoptable");
        return;
    }
    #[cfg(feature = "nonos-smp")]
    match crate::smp::start_aps() {
        /*
         * Assembled whole, then emitted once. An application processor prints
         * its own bring-up at the same moment, and these used to interleave
         * into `cpu_count=[APIC] Setting up timer at 1100 UP`. The boot matrix
         * reads this line to decide whether the cell passed, so a shredded one
         * is a cell graded on a string that was never printed.
         */
        Ok(started) => {
            let mut l = crate::sys::serial::Line::new();
            l.str(b"[SMP-PROOF] cpu_count=").dec((started + 1) as u64);
            l.str(if started > 0 { b" PASS" } else { b" UP" });
            l.end();
        }
        Err(e) => {
            let mut l = crate::sys::serial::Line::new();
            l.str(b"[SMP-PROOF] FAIL ").str(e.as_bytes());
            l.end();
        }
    }
}
