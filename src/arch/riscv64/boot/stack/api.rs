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

use super::{register, state};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackError {
    InvalidHart,
}

pub type StackResult<T> = Result<T, StackError>;

pub fn setup_stack(hart_id: usize) -> StackResult<()> {
    let stack_top = state::kernel_top(hart_id).ok_or(StackError::InvalidHart)?;
    register::switch_to(stack_top);
    Ok(())
}

pub fn get_kernel_stack(hart_id: usize) -> Option<u64> {
    state::kernel_top(hart_id)
}

pub fn get_irq_stack(hart_id: usize) -> Option<u64> {
    state::irq_top(hart_id)
}

pub fn current_stack_pointer() -> u64 {
    register::current_stack_pointer()
}

pub fn stack_remaining(hart_id: usize) -> Option<usize> {
    let sp = current_stack_pointer();
    let base = state::kernel_base(hart_id)?;
    Some(sp.saturating_sub(base) as usize)
}
