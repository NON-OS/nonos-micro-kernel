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

use alloc::vec;

use super::types::Terminal;
use crate::rail::Rail;
use crate::term::dimensions::{MAX_FONT_SCALE, MIN_FONT_SCALE};
use crate::term::prefs::store;
use crate::term::state::State;
use crate::term::theme::profiles;

impl Terminal {
    pub fn new() -> Self {
        let prefs = store::load();
        Self {
            tabs: vec![State::new()],
            active: 0,
            width: 0,
            acc_w: 0,
            theme: prefs.theme.min(profiles::COUNT - 1),
            font_scale: (prefs.font_scale as u32).clamp(MIN_FONT_SCALE, MAX_FONT_SCALE),
            prefs,
            prefs_dirty: false,
            prefs_ticks: 0,
            rail: Rail::new(),
        }
    }
}
