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

use super::constants::MAX_INTERRUPTS;
use super::{state, Plic, PlicResult};

impl Plic {
    pub fn init(&self) -> PlicResult<()> {
        for irq in 1..MAX_INTERRUPTS {
            self.set_priority(irq, 0)?;
        }
        self.set_threshold(super::super::super::cpu::hart_id(), 0)
    }
}

pub fn init_plic(base: u64) -> PlicResult<()> {
    state::install(base)?;
    Plic::new(base).init()
}
