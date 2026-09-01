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

use alloc::vec::Vec;

use crate::term::state::State;

pub struct Terminal {
    pub(crate) tabs: Vec<State>,
    pub(crate) active: usize,
    // Window width from the last paint, so the titlebar accessory can ask for a
    // width that stays clear of the traffic lights.
    pub(crate) width: u32,
    // Accessory width the frame actually granted, recorded when it hands over
    // the sub-buffer so the painter and the hit-test share one geometry.
    pub(crate) acc_w: u32,
    // Window level, so a new tab inherits the look instead of resetting it.
    pub(crate) theme: u16,
    pub(crate) font_scale: u32,
}

impl Terminal {
    pub(crate) fn cur(&mut self) -> &mut State {
        &mut self.tabs[self.active]
    }
    pub(crate) fn cur_ref(&self) -> &State {
        &self.tabs[self.active]
    }
}
