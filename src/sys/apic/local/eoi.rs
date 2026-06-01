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

use super::constants::LAPIC_EOI;
use super::regs::lapic_write_raw;

// Acknowledge the in-service vector. Hot-path: called from every IRQ
// handler. No branching beyond the volatile write; the LAPIC_INIT
// guard is intentionally absent because writing to a not-yet-mapped
// LAPIC would already have faulted long before the first interrupt.
#[inline]
pub fn eoi() {
    unsafe {
        lapic_write_raw(LAPIC_EOI, 0);
    }
}
