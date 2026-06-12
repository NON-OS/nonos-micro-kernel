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

use super::role::Role;
use crate::term::dimensions::{COLS, SCROLLBACK_ROWS};

pub struct Scrollback {
    pub(super) rows: [[u8; COLS]; SCROLLBACK_ROWS],
    pub(super) lengths: [u16; SCROLLBACK_ROWS],
    pub(super) roles: [Role; SCROLLBACK_ROWS],
    pub(super) head: usize,
    pub(super) count: usize,
    pub(super) view_offset: usize,
    // When `Some`, `push_line` diverts each line here instead of the
    // visible ring. Used to capture a command's output for redirection
    // (`cmd > file`) without showing it on screen.
    pub(super) capture: Option<Vec<Vec<u8>>>,
}
