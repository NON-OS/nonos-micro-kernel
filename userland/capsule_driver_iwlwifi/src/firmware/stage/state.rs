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

#[derive(Clone, Copy)]
pub struct FirmwareStageState {
    pub major: u16,
    pub minor: u16,
    pub api: u16,
    pub build: u32,
    pub init_sections: u16,
    pub runtime_sections: u16,
    pub paging_sections: u16,
    pub staged_bytes: u32,
    pub alive_seen: bool,
    pub last_int: u32,
}
