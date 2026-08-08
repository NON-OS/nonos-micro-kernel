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

//! Enhanced Configuration Access: config space as memory.
//!
//! Every function gets a 4 KiB page, addressed by concatenating the bus,
//! device and function numbers below the register offset. No lock is needed
//! and no read-modify-write either, unlike the port pair: a byte store here is
//! a byte store on the bus, which matters for the registers whose bits are
//! cleared by writing one to them. Doing those through a dword
//! read-modify-write would clear neighbouring bits as a side effect.

use core::sync::atomic::{AtomicU64, Ordering};

/// Physical base of the window, or zero when the platform published none.
static BASE: AtomicU64 = AtomicU64::new(0);
/// Extent of the window, used to reject an access that would fall outside it.
static SIZE: AtomicU64 = AtomicU64::new(0);

const BUS_SHIFT: u64 = 20;
const DEVICE_SHIFT: u64 = 15;
const FUNCTION_SHIFT: u64 = 12;

/// Record the window the firmware or device tree described.
///
/// `base` must already be mapped as device memory by the caller; a `size` of
/// zero means there is no window, which leaves ECAM unconfigured.
pub fn set_ecam_window(base: u64, size: u64) {
    BASE.store(base, Ordering::Relaxed);
    SIZE.store(size, Ordering::Release);
}

pub(super) fn is_configured() -> bool {
    SIZE.load(Ordering::Acquire) != 0
}

/// The address of one config register, or `None` if it falls outside the
/// window the platform published.
fn address_of(bus: u8, device: u8, function: u8, offset: u16) -> Option<u64> {
    let displacement = ((bus as u64) << BUS_SHIFT)
        | ((device as u64) << DEVICE_SHIFT)
        | ((function as u64) << FUNCTION_SHIFT)
        | offset as u64;
    if displacement >= SIZE.load(Ordering::Acquire) {
        return None;
    }
    Some(BASE.load(Ordering::Relaxed) + displacement)
}

/// Generate a read and a write for one access width.
///
/// A read outside the window answers all-ones, which is what the bus itself
/// returns for a function nothing implements, so callers need no new case. A
/// write outside it is dropped rather than aimed at whatever happens to follow
/// the window in physical memory.
macro_rules! accessor {
    ($read:ident, $write:ident, $ty:ty) => {
        pub(super) fn $read(bus: u8, device: u8, function: u8, offset: u16) -> $ty {
            match address_of(bus, device, function, offset) {
                // SAFETY: the address is inside the mapped ECAM window, and
                // config space tolerates a read of any width at a naturally
                // aligned offset. The caller validated the alignment.
                Some(addr) => unsafe { core::ptr::read_volatile(addr as *const $ty) },
                None => !0,
            }
        }

        pub(super) fn $write(bus: u8, device: u8, function: u8, offset: u16, value: $ty) {
            if let Some(addr) = address_of(bus, device, function, offset) {
                // SAFETY: as for the read above.
                unsafe {
                    core::ptr::write_volatile(addr as *mut $ty, value);
                }
            }
        }
    };
}

accessor!(read8, write8, u8);
accessor!(read16, write16, u16);
accessor!(read32, write32, u32);
