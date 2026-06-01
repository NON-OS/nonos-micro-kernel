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
use super::io::RegIo;
#[derive(Clone, Copy)]
pub struct Regs {
    pub(super) common: RegIo,
    pub(super) common_offset: usize,
    pub(super) notify: RegIo,
    pub(super) notify_offset: usize,
    pub(super) notify_multiplier: usize,
    pub(super) device: RegIo,
    pub(super) device_offset: usize,
}
impl Regs {
    pub const fn mmio(base: u64) -> Self {
        let io = RegIo::Mmio(base as *mut u8);
        Self::from_parts(io, 0, io, crate::constants::LEG_QUEUE_NOTIFY, 0, io, 0)
    }
    pub const fn modern(
        common: u64,
        common_offset: usize,
        notify: u64,
        notify_offset: usize,
        notify_multiplier: usize,
        device: u64,
        device_offset: usize,
    ) -> Self {
        Self::from_parts(
            RegIo::Mmio(common as *mut u8),
            common_offset,
            RegIo::Mmio(notify as *mut u8),
            notify_offset,
            notify_multiplier,
            RegIo::Mmio(device as *mut u8),
            device_offset,
        )
    }
    pub const fn pio(grant_id: u64) -> Self {
        let io = RegIo::Pio(grant_id);
        Self::from_parts(io, 0, io, crate::constants::LEG_QUEUE_NOTIFY, 0, io, 0)
    }
    pub const fn with_queue_notify(self, queue_notify: u16) -> Self {
        Self { notify_offset: self.notify_offset + queue_notify as usize * self.notify_multiplier, ..self }
    }
    const fn from_parts(
        common: RegIo,
        common_offset: usize,
        notify: RegIo,
        notify_offset: usize,
        notify_multiplier: usize,
        device: RegIo,
        device_offset: usize,
    ) -> Self {
        Self { common, common_offset, notify, notify_offset, notify_multiplier, device, device_offset }
    }
}
