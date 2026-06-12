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

use super::role::Role;
use super::types::Scrollback;
use crate::term::dimensions::{COLS, SCROLLBACK_ROWS};

impl Scrollback {
    pub const fn new() -> Self {
        Self {
            rows: [[0; COLS]; SCROLLBACK_ROWS],
            lengths: [0; SCROLLBACK_ROWS],
            roles: [Role::Normal; SCROLLBACK_ROWS],
            head: 0,
            count: 0,
            view_offset: 0,
            capture: None,
        }
    }
}
