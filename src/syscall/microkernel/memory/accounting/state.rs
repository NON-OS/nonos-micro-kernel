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

use core::sync::atomic::{AtomicU64, AtomicUsize};
use spin::Mutex;

pub(super) const OWNER_CAP: usize = 64;
pub(super) const EVENT_CAP: usize = 20;
pub(super) const OWNER_ZERO: Owner = Owner { pid: 0, bytes: 0 };
pub(super) const EVENT_ZERO: Event = Event { pid: 0, kind: 0, size: 0, va: 0, owner: 0, system: 0 };

#[derive(Clone, Copy)]
pub(super) struct Owner {
    pub pid: u32,
    pub bytes: u64,
}

#[derive(Clone, Copy)]
pub(super) struct Event {
    pub pid: u32,
    pub kind: u8,
    pub size: u64,
    pub va: u64,
    pub owner: u64,
    pub system: u64,
}

pub(super) static OWNERS: Mutex<[Owner; OWNER_CAP]> = Mutex::new([OWNER_ZERO; OWNER_CAP]);
pub(super) static EVENTS: Mutex<[Event; EVENT_CAP]> = Mutex::new([EVENT_ZERO; EVENT_CAP]);
pub(super) static EVENT_CURSOR: AtomicUsize = AtomicUsize::new(0);
pub(super) static SYSTEM_BYTES: AtomicU64 = AtomicU64::new(0);
