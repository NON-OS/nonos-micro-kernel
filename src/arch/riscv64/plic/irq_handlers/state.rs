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

use core::sync::atomic::{AtomicPtr, AtomicU8};

pub const MAX_IRQ: u32 = 1024;

pub(super) static IRQ_HANDLERS: [AtomicPtr<()>; MAX_IRQ as usize] = {
    const INIT: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());
    [INIT; MAX_IRQ as usize]
};

pub(super) const OWNER_FREE: u8 = 0;
pub(super) const OWNER_KERNEL: u8 = 1;
pub(super) const OWNER_CAPSULE: u8 = 2;

pub(super) static IRQ_OWNERS: [AtomicU8; MAX_IRQ as usize] = {
    const INIT: AtomicU8 = AtomicU8::new(OWNER_FREE);
    [INIT; MAX_IRQ as usize]
};
