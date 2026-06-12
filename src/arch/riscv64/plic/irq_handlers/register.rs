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

use core::sync::atomic::Ordering;

use super::state::{IRQ_HANDLERS, IRQ_OWNERS, MAX_IRQ, OWNER_CAPSULE, OWNER_FREE, OWNER_KERNEL};

#[derive(Debug, Clone, Copy)]
pub enum PlicIrqError {
    OutOfRange,
    AlreadyClaimed,
    NotOwner,
}

pub fn register(irq: u32, handler: fn(u32)) -> Result<(), PlicIrqError> {
    claim(irq, handler, OWNER_KERNEL)
}

pub fn register_for_capsule(irq: u32, handler: fn(u32)) -> Result<(), PlicIrqError> {
    claim(irq, handler, OWNER_CAPSULE)
}

pub fn unregister(irq: u32) -> Result<(), PlicIrqError> {
    release(irq, OWNER_KERNEL)
}

pub fn unregister_for_capsule(irq: u32) -> Result<(), PlicIrqError> {
    release(irq, OWNER_CAPSULE)
}

fn claim(irq: u32, handler: fn(u32), owner: u8) -> Result<(), PlicIrqError> {
    if irq == 0 || irq >= MAX_IRQ {
        return Err(PlicIrqError::OutOfRange);
    }
    IRQ_OWNERS[irq as usize]
        .compare_exchange(OWNER_FREE, owner, Ordering::AcqRel, Ordering::Acquire)
        .map_err(|_| PlicIrqError::AlreadyClaimed)?;
    IRQ_HANDLERS[irq as usize].store(handler as *mut (), Ordering::Release);
    Ok(())
}

fn release(irq: u32, expected: u8) -> Result<(), PlicIrqError> {
    if irq == 0 || irq >= MAX_IRQ {
        return Err(PlicIrqError::OutOfRange);
    }
    if IRQ_OWNERS[irq as usize].load(Ordering::Acquire) != expected {
        return Err(PlicIrqError::NotOwner);
    }
    IRQ_HANDLERS[irq as usize].store(core::ptr::null_mut(), Ordering::Release);
    IRQ_OWNERS[irq as usize].store(OWNER_FREE, Ordering::Release);
    Ok(())
}
