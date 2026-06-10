// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::menu::SecurityMode;

pub fn mode_from(v: u8) -> SecurityMode {
    match v {
        2 => SecurityMode::Hardened,
        3 => SecurityMode::SafeMode,
        4 => SecurityMode::NetworkIsolated,
        5 => SecurityMode::Recovery,
        _ => SecurityMode::Standard,
    }
}

pub fn next_mode(v: u8) -> u8 {
    if v >= 5 {
        1
    } else {
        v + 1
    }
}

pub fn mode_name(v: u8) -> &'static str {
    mode_from(v).label()
}
