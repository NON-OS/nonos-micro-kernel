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
use crate::protocol::E_AUTH;

use super::{Context, SessionState};

impl Context {
    pub fn end_session(&mut self, caller_pid: u32) -> Result<(), i32> {
        match self.state {
            SessionState::Locked => Ok(()),
            SessionState::Unlocked { owner_pid, .. } if owner_pid != caller_pid => Err(E_AUTH),
            SessionState::Unlocked { .. } => {
                self.state = SessionState::Locked;
                Ok(())
            }
        }
    }
}
