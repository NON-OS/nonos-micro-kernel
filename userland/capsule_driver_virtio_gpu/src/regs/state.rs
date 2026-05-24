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
    pub(super) io: RegIo,
    pub(super) notify_offset: usize,
}
impl Regs {
    pub const fn mmio(base: u64) -> Self {
        Self { io: RegIo::Mmio(base as *mut u8), notify_offset: crate::constants::LEG_QUEUE_NOTIFY }
    }
    pub const fn mmio_with_notify(base: u64, notify_offset: usize) -> Self {
        Self { io: RegIo::Mmio(base as *mut u8), notify_offset }
    }
    pub const fn pio(grant_id: u64) -> Self {
        Self { io: RegIo::Pio(grant_id), notify_offset: crate::constants::LEG_QUEUE_NOTIFY }
    }
}
