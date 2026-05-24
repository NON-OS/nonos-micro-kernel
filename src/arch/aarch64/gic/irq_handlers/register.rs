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

use super::state::{
    IRQ_HANDLERS, IRQ_OWNERS, MAX_INTID, OWNER_CAPSULE, OWNER_FREE, OWNER_KERNEL,
};

#[derive(Debug, Clone, Copy)]
pub enum GicIrqError {
    OutOfRange,
    AlreadyClaimed,
    NotOwner,
}



pub fn register(intid: u32, handler: fn(u32)) -> Result<(), GicIrqError> {
    claim(intid, handler, OWNER_KERNEL)
}



pub fn register_for_capsule(intid: u32, handler: fn(u32)) -> Result<(), GicIrqError> {
    claim(intid, handler, OWNER_CAPSULE)
}

pub fn unregister(intid: u32) -> Result<(), GicIrqError> {
    release(intid, OWNER_KERNEL)
}

pub fn unregister_for_capsule(intid: u32) -> Result<(), GicIrqError> {
    release(intid, OWNER_CAPSULE)
}

fn claim(intid: u32, handler: fn(u32), owner: u8) -> Result<(), GicIrqError> {
    if intid >= MAX_INTID {
        return Err(GicIrqError::OutOfRange);
    }
    IRQ_OWNERS[intid as usize]
        .compare_exchange(OWNER_FREE, owner, Ordering::AcqRel, Ordering::Acquire)
        .map_err(|_| GicIrqError::AlreadyClaimed)?;
    IRQ_HANDLERS[intid as usize].store(handler as *mut (), Ordering::Release);
    Ok(())
}




fn release(intid: u32, expected: u8) -> Result<(), GicIrqError> {
    if intid >= MAX_INTID {
        return Err(GicIrqError::OutOfRange);
    }
    if IRQ_OWNERS[intid as usize].load(Ordering::Acquire) != expected {
        return Err(GicIrqError::NotOwner);
    }
    IRQ_HANDLERS[intid as usize].store(core::ptr::null_mut(), Ordering::Release);
    IRQ_OWNERS[intid as usize].store(OWNER_FREE, Ordering::Release);
    Ok(())
}
