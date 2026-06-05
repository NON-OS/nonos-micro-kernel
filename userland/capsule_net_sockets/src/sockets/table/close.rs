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

use crate::sockets::SocketKey;

use super::types::Table;

impl Table {
    pub fn close(&self, key: SocketKey) -> bool {
        let mut g = self.inner.lock();
        for slot in g.iter_mut() {
            if slot.as_ref().map(|s| s.key.handle == key.handle && s.key.pid == key.pid)
                == Some(true)
            {
                *slot = None;
                return true;
            }
        }
        false
    }
}
