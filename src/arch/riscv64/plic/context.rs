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

use super::registers::{current_plic, set_threshold, PlicResult};

pub struct PlicContext {
    hart: usize,
    context_id: usize,
}

impl PlicContext {
    pub fn new(hart: usize) -> Self {
        Self { hart, context_id: hart * 2 + 1 }
    }

    pub fn hart(&self) -> usize {
        self.hart
    }

    pub fn context_id(&self) -> usize {
        self.context_id
    }

    pub fn enable_irq(&self, irq: u32) -> PlicResult<()> {
        current_plic()?.enable(self.hart, irq)
    }

    pub fn disable_irq(&self, irq: u32) -> PlicResult<()> {
        current_plic()?.disable(self.hart, irq)
    }

    pub fn set_threshold(&self, threshold: u8) -> PlicResult<()> {
        set_threshold(self.hart, threshold)
    }

    pub fn claim(&self) -> PlicResult<Option<u32>> {
        current_plic()?.claim(self.hart)
    }

    pub fn complete(&self, irq: u32) -> PlicResult<()> {
        current_plic()?.complete(self.hart, irq)
    }
}

pub fn init_plic_hart() -> PlicResult<()> {
    let hart = super::super::cpu::hart_id();
    let ctx = PlicContext::new(hart);
    ctx.set_threshold(0)
}

pub fn current_context() -> PlicContext {
    let hart = super::super::cpu::hart_id();
    PlicContext::new(hart)
}
