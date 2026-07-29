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

use super::state::{get_port, primary_port_index, SerialStatsSnapshot};

pub fn get_stats(port_index: usize) -> Option<SerialStatsSnapshot> {
    let state = get_port(port_index)?;
    Some(state.stats.snapshot())
}

pub fn get_primary_stats() -> SerialStatsSnapshot {
    let port_index = primary_port_index();
    get_stats(port_index).unwrap_or_default()
}

pub fn reset_stats(port_index: usize) -> bool {
    if let Some(state) = get_port(port_index) {
        state.stats.reset();
        true
    } else {
        false
    }
}
