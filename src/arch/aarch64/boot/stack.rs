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

mod api;
mod register;
mod state;
mod types;

pub use api::{
    current_stack_pointer, get_exception_stack, get_irq_stack, get_kernel_stack, setup_stack,
    stack_remaining, StackError, StackResult,
};
pub use state::MAX_CPUS;
pub use types::{
    ExceptionStack, IrqStack, KernelStack, EXCEPTION_STACK_SIZE, IRQ_STACK_SIZE, KERNEL_STACK_SIZE,
};
