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

use core::cell::UnsafeCell;

use super::types::{ExceptionStack, IrqStack, KernelStack};

pub const MAX_CPUS: usize = 256;

struct StackBank<T, const N: usize> {
    slots: UnsafeCell<[T; N]>,
}

unsafe impl<T, const N: usize> Sync for StackBank<T, N> {}

impl<T, const N: usize> StackBank<T, N> {
    const fn new(slots: [T; N]) -> Self {
        Self { slots: UnsafeCell::new(slots) }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= N {
            None
        } else {
            unsafe { Some(&(*self.slots.get())[index]) }
        }
    }
}

static KERNEL_STACKS: StackBank<KernelStack, MAX_CPUS> =
    StackBank::new([const { KernelStack::new() }; MAX_CPUS]);
static IRQ_STACKS: StackBank<IrqStack, MAX_CPUS> =
    StackBank::new([const { IrqStack::new() }; MAX_CPUS]);
static EXCEPTION_STACKS: StackBank<ExceptionStack, MAX_CPUS> =
    StackBank::new([const { ExceptionStack::new() }; MAX_CPUS]);

pub fn kernel_top(cpu_id: usize) -> Option<u64> {
    KERNEL_STACKS.get(cpu_id).map(KernelStack::top)
}

pub fn kernel_base(cpu_id: usize) -> Option<u64> {
    KERNEL_STACKS.get(cpu_id).map(KernelStack::base)
}

pub fn irq_top(cpu_id: usize) -> Option<u64> {
    IRQ_STACKS.get(cpu_id).map(IrqStack::top)
}

pub fn exception_top(cpu_id: usize) -> Option<u64> {
    EXCEPTION_STACKS.get(cpu_id).map(ExceptionStack::top)
}
