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

use super::types::Rte;

impl Rte {
    pub(crate) fn flags_bits(self) -> u32 {
        let mut f = 0u32;
        if self.logical {
            f |= 1 << 0;
        }
        if self.active_low {
            f |= 1 << 1;
        }
        if self.level_trigger {
            f |= 1 << 2;
        }
        if self.masked {
            f |= 1 << 3;
        }
        f | ((self.delivery as u32) << 8)
    }
}
