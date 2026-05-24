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

// Write CR3. Flushes the non-global TLB on retire. The next
// instruction fetch goes through the new mapping, so the caller
// has to make sure RIP and RSP are still reachable in `cr3`.
#[inline]
pub(super) unsafe fn write_cr3(cr3: u64) {
    core::arch::asm!(
        "mov {0}, %cr3",
        in(reg) cr3,
        options(att_syntax, nostack, preserves_flags)
    );
}
