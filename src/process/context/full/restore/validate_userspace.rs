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

use super::super::definition::Context;

impl Context {
    pub fn validate_userspace(&self) -> Result<(), &'static str> {
        self.validate()?;
        if !Self::is_user_space_addr(self.rip) {
            return Err("RIP not in user space");
        }
        if !Self::is_user_space_addr(self.rsp) {
            return Err("RSP not in user space");
        }
        Ok(())
    }
}
