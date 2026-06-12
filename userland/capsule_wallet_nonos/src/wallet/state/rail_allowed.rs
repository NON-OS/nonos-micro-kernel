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

use super::types::Rail;

pub fn rail_allowed(rail: &Rail) -> bool {
    match rail.symbol_len {
        2 => &rail.symbol[..2] == b"PR",
        3 => &rail.symbol[..3] == b"ETH" || &rail.symbol[..3] == b"NOX",
        _ => false,
    }
}
