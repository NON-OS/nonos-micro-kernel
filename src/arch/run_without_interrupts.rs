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

// Runs `f` with interrupts masked and restores the caller's mask afterwards, so
// a nested scope does not re-enable them early. This goes through `ArchOps`,
// which every architecture implements: the previous form ran `f` untouched
// anywhere but x86_64, meaning a critical section on another architecture
// executed with interrupts still enabled.
//
// The kernel aborts on panic, so no unwind path can skip the restore.
pub(crate) fn run_without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    use crate::arch::abi::ArchOps;

    let was_enabled = crate::arch::Arch::interrupts_enabled();
    if was_enabled {
        // SAFETY: masking interrupts cannot itself violate memory safety, and
        // the matching restore below returns the CPU to the state the caller
        // was in. Only the mask taken here is released, so an outer scope that
        // had already masked them keeps its mask.
        unsafe { crate::arch::Arch::disable_interrupts() };
    }
    let result = f();
    if was_enabled {
        // SAFETY: restores exactly the mask this call took, and runs only when
        // interrupts were enabled on entry.
        unsafe { crate::arch::Arch::enable_interrupts() };
    }
    result
}
