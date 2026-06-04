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
    pub const fn pio(grant_id: u64) -> Self {
        let io = RegIo::Pio(grant_id);
        Self {
            common: io,
            common_offset: 0,
            notify: io,
            notify_offset: crate::constants::LEG_QUEUE_NOTIFY,
            notify_multiplier: 0,
            device: io,
            device_offset: 0,
        }
    }
}
