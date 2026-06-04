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
use crate::protocol::E_BUSY;

use super::{Context, SessionState};

impl Context {
    pub fn start_session(&mut self, owner_pid: u32, key_id: u32) -> Result<u32, i32> {
        if matches!(self.state, SessionState::Unlocked { .. }) {
            return Err(E_BUSY);
        }
        self.serial = self.serial.wrapping_add(1);
        let serial = self.serial;
        self.state = SessionState::Unlocked { owner_pid, key_id, serial };
        Ok(serial)
    }
}
