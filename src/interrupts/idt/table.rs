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

//! The one interrupt descriptor table this kernel runs on, built once and
//! loaded by every CPU. The vectors are grouped by what is different about
//! them rather than by number: those that only need a swapgs trampoline, those
//! that also need a stack of their own, and the device and syscall lines.

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = build_idt();
}

fn build_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();
    // SAFETY: ek@nonos.systems - each group installs addresses of naked or
    // `x86-interrupt` functions pinned in kernel text, and the IST indices
    // name stacks the per-CPU TSS already carries. Their own docs carry the
    // per-group argument.
    unsafe {
        super::table_trampolines::configure(&mut idt);
        super::table_ist::configure(&mut idt);
        super::table_irqs::configure(&mut idt);
    }
    idt
}
