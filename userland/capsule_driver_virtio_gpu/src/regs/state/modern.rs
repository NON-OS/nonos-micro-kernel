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
use super::super::io::RegIo;
use super::types::Regs;

impl Regs {
    pub const fn modern(
        common: u64,
        common_offset: usize,
        notify: u64,
        notify_offset: usize,
        notify_multiplier: usize,
        device: u64,
        device_offset: usize,
    ) -> Self {
        Self {
            common: RegIo::Mmio(common as *mut u8),
            common_offset,
            notify: RegIo::Mmio(notify as *mut u8),
            notify_offset,
            notify_multiplier,
            device: RegIo::Mmio(device as *mut u8),
            device_offset,
        }
    }
}
