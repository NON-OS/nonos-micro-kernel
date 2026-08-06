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

use super::types::Table;
use crate::state::Session;

impl Table {
    /// The session running over Sphinx, if exactly one is bound.
    pub fn with_sphinx_session<R>(&mut self, f: impl FnOnce(&mut Session) -> R) -> Option<R> {
        let mut bound = self.sessions.iter_mut().filter(|s| s.dest != [0u8; 32]);
        let first = bound.next()?;
        if bound.next().is_some() {
            return None;
        }
        Some(f(first))
    }

    pub fn sphinx_session_count(&self) -> usize {
        self.sessions.iter().filter(|s| s.dest != [0u8; 32]).count()
    }

    pub fn session_has_dest(&self, owner: u32, id: u32) -> bool {
        self.sessions.iter().any(|s| s.owner == owner && s.id == id && s.dest != [0u8; 32])
    }
}
