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

use super::{state, PlicResult};

pub fn enable_irq(irq: u32) -> PlicResult<()> {
    let plic = state::current_plic()?;
    let hart = super::super::super::cpu::hart_id();
    plic.set_priority(irq, 1)?;
    plic.enable(hart, irq)
}

pub fn disable_irq(irq: u32) -> PlicResult<()> {
    let plic = state::current_plic()?;
    plic.disable(super::super::super::cpu::hart_id(), irq)
}

pub fn set_priority(irq: u32, priority: u32) -> PlicResult<()> {
    let plic = state::current_plic()?;
    plic.set_priority(irq, priority as u8)
}

pub fn set_threshold(hart: usize, threshold: u8) -> PlicResult<()> {
    state::current_plic()?.set_threshold(hart, threshold)
}

pub fn claim_interrupt() -> PlicResult<Option<u32>> {
    state::current_plic()?.claim(super::super::super::cpu::hart_id())
}

pub fn complete_interrupt(irq: u32) -> PlicResult<()> {
    state::current_plic()?.complete(super::super::super::cpu::hart_id(), irq)
}
