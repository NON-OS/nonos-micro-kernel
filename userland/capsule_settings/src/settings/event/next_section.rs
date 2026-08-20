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

use crate::settings::section::{Section, SECTIONS};
use crate::settings::state::{set_section, State};

/// Step through the sidebar in the order it is drawn, wrapping at both ends.
pub fn next_section(state: &mut State) {
    step(state, 1);
}

pub fn prev_section(state: &mut State) {
    step(state, -1);
}

fn step(state: &mut State, delta: i32) {
    let n = SECTIONS.len() as i32;
    let i = state.section.index() as i32;
    let next = ((i + delta) % n + n) % n;
    set_section(state, Section::from_index(next as usize));
}
