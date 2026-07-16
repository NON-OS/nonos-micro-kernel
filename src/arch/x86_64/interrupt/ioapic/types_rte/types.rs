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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rte {
    pub vector: u8,
    pub delivery: u8,
    pub logical: bool,
    pub active_low: bool,
    pub level_trigger: bool,
    pub masked: bool,
    pub dest_apic_id: u32,
}

impl Default for Rte {
    fn default() -> Self {
        Self::fixed(0, 0)
    }
}
