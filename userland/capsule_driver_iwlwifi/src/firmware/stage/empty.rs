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

use super::state::FirmwareStageState;

impl FirmwareStageState {
    pub const fn empty() -> Self {
        Self {
            major: 0,
            minor: 0,
            api: 0,
            build: 0,
            init_sections: 0,
            runtime_sections: 0,
            paging_sections: 0,
            staged_bytes: 0,
            alive_seen: false,
            last_int: 0,
        }
    }
}
