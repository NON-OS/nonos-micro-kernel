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

use super::body;
use super::header;
use super::scrollbar;
use super::status_bar;
use super::tabs;
use crate::about::state::State;
use crate::about::theme::BACKGROUND;

pub fn paint(state: &mut State, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    header::paint(fb);
    tabs::paint(state, fb);
    body::paint(state, fb);
    scrollbar::paint(state, fb);
    status_bar::paint(fb);
    state.painted = true;
}
