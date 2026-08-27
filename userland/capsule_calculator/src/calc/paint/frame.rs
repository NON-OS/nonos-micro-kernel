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

use nonos_app_skeleton::PaintBuffer;

use super::background;
use super::display;
use super::grid;
use super::memory_badge;
use super::rail;
use super::wordmark;
use crate::calc::state::State;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    background::paint(fb);
    wordmark::paint(fb);
    display::paint(state, fb);
    memory_badge::paint(state, fb);
    grid::paint(fb);
    rail::paint(state, fb);
}
