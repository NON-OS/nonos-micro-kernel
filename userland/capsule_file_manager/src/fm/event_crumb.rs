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

extern crate alloc;

use alloc::string::String;

use super::crumbs::crumb_parts;
use super::navigate::navigate;
use super::state::State;

/// Navigate to the prefix the crumb at `index` names. Index 0 is the root
/// segment, which the painter draws as a home glyph rather than a bare slash, so
/// it contributes no path component of its own.
pub fn crumb_nav(state: &mut State, index: usize) {
    let parts = crumb_parts(state);
    let mut path = String::from("/");
    for part in parts.iter().take(index + 1).skip(1) {
        path.push_str(part.as_str());
        path.push('/');
    }
    navigate(state, path.as_str());
}
