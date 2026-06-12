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
use crate::term::dimensions::SCROLLBACK_ROWS;

pub struct ScrollbackView<'a> {
    pub(super) sb: &'a Scrollback,
    pub(super) start: usize,
    pub(super) end: usize,
}

impl<'a> ScrollbackView<'a> {
    pub fn rows(&self) -> impl Iterator<Item = (&'a [u8], Role)> + '_ {
        let head = self.sb.head;
        let lengths = &self.sb.lengths;
        let rows = &self.sb.rows;
        let roles = &self.sb.roles;
        (self.start..self.end).map(move |logical| {
            let slot = (head + logical) % SCROLLBACK_ROWS;
            let n = lengths[slot] as usize;
            (&rows[slot][..n], roles[slot])
        })
    }
}
