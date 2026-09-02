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

//! Bringing the ring-0 restrictions up from the hardening side.
//!
//! This used to write CR4 and CR0 itself, in parallel with `memory::mmu`,
//! which meant two pieces of code owned the same registers and neither read
//! back what it had set. There is one owner now and this delegates to it, so
//! however this path is reached the machine ends up in the state the boot log
//! reported rather than in whichever state ran last.

use crate::memory::mmu;

/// Idempotent: `init_mmu` returns having touched nothing if the bring-up
/// already ran from the boot path, which is the usual case.
pub fn init_module_memory_protection() {
    let _ = mmu::init_mmu();
}
