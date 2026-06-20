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

use super::role::Role;
use super::types::Scrollback;

impl Scrollback {
    pub(super) fn push_raw(&mut self, line: &[u8], role: Role) {
        match role {
            Role::Error => {
                self.grid.feed(b"\x1b[31m");
                self.grid.feed(line);
                self.grid.feed(b"\x1b[0m\n");
            }
            Role::Normal => {
                self.grid.feed(line);
                self.grid.feed(b"\n");
            }
        }
    }
}
