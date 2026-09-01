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

use super::entry::{Action, Kind};
use super::index::Index;
use crate::paint::rail_row::base_name;
use crate::term::state::State;

const HISTORY_MAX: usize = 12;

fn text(b: &[u8]) -> &str {
    core::str::from_utf8(b).unwrap_or("")
}

impl<'a> Index<'a> {
    /// Newest first, walked with the same `search_back` the reverse-i-search
    /// uses, so the palette and Ctrl+R can never disagree about what ran. An
    /// empty needle takes the newest line at or before the cursor, and the
    /// returned position becomes the next cursor.
    pub(super) fn history(&mut self, s: &'a State) {
        let mut before = usize::MAX;
        for _ in 0..HISTORY_MAX {
            let Some((at, line)) = s.history.search_back(&[], before) else {
                return;
            };
            self.push(Kind::History, text(line), "run again", Action::Run);
            if at == 0 {
                return;
            }
            before = at;
        }
    }

    pub(super) fn sessions(&mut self, tabs: &'a [State]) {
        for (i, t) in tabs.iter().enumerate() {
            let label = base_name(text(t.cwd.as_bytes()));
            self.push(Kind::Session, label, "switch to", Action::Session(i as u16));
        }
    }
}
