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

// Split "12.5px" into its number part when the value ends in `unit`.
// Non-boundary or empty splits reject the value.
pub(super) fn strip_unit<'a>(v: &'a str, unit: &str) -> Option<&'a str> {
    let cut = v.len().checked_sub(unit.len())?;
    if cut == 0 {
        return None;
    }
    if v.get(cut..)?.eq_ignore_ascii_case(unit) {
        v.get(..cut)
    } else {
        None
    }
}
