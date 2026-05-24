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

use super::registers::{read_pc, read_sp, read_spsr_el1};

#[derive(Debug, Clone, Copy, Default)]
pub struct CpuState {
    pub x: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
}

impl CpuState {
    pub fn capture() -> Self {
        let mut state = Self::default();
        state.sp = read_sp();
        state.pc = read_pc();
        state.pstate = read_spsr_el1();
        state
    }
}
