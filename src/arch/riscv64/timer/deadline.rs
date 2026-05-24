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

use crate::arch::riscv64::sbi::SbiError;

use super::read::read_time;

const DEFAULT_TICK_DELTA: u64 = 10_000_000;

pub fn init_timer() -> Result<(), SbiError> {
    set_next_timer(DEFAULT_TICK_DELTA)
}

pub fn init_timer_hart() -> Result<(), SbiError> {
    set_next_timer(DEFAULT_TICK_DELTA)
}

pub fn set_next_timer(ticks: u64) -> Result<(), SbiError> {
    super::super::sbi::set_timer(read_time().saturating_add(ticks))
}

pub(super) const fn default_tick_delta() -> u64 {
    DEFAULT_TICK_DELTA
}
