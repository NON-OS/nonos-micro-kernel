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

use crate::protocol::TRAY_LABEL_MAX;

pub const MAX_TRAY_ITEMS: usize = 32;

#[derive(Clone, Copy)]
pub struct TrayEntry {
    pub owner_pid: u32,
    pub tray_id: u32,
    pub label_len: u32,
    pub label: [u8; TRAY_LABEL_MAX],
    pub in_use: bool,
}

impl Default for TrayEntry {
    fn default() -> Self {
        Self { owner_pid: 0, tray_id: 0, label_len: 0, label: [0; TRAY_LABEL_MAX], in_use: false }
    }
}
