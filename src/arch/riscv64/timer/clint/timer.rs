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

use super::state::current_clint;

pub fn set_timer_interrupt(ticks: u64) {
    let clint = current_clint();
    let hart = super::super::super::cpu::hart_id();
    clint.set_mtimecmp(hart, clint.mtime().saturating_add(ticks));
}

pub fn clear_timer_interrupt() {
    current_clint().set_mtimecmp(super::super::super::cpu::hart_id(), u64::MAX);
}

pub fn read_mtime() -> u64 {
    current_clint().mtime()
}
