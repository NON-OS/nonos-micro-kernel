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

use super::super::critical::is_critical;
use super::super::security::sensitive::ANY;
use super::{Row, State};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Elevated,
    Protected,
    Flagged,
}

pub const FILTERS: [Filter; 4] =
    [Filter::All, Filter::Elevated, Filter::Protected, Filter::Flagged];

impl Filter {
    pub fn label(self) -> &'static [u8] {
        match self {
            Filter::All => b"All",
            Filter::Elevated => b"Elevated",
            Filter::Protected => b"Protected",
            Filter::Flagged => b"Flagged",
        }
    }

    // `flagged` is the set of pids the security monitor named this refresh.
    pub fn keep(self, row: &Row, flagged: &[u32]) -> bool {
        match self {
            Filter::All => true,
            Filter::Elevated => row.caps & ANY != 0,
            Filter::Protected => is_critical(row.name()),
            Filter::Flagged => flagged.contains(&row.pid),
        }
    }
}

impl State {
    // The rows the user can currently see: the sort order from refresh(),
    // narrowed by the active filter and the search query. Painters and the hit
    // test both index into this, never into `rows`, or a click lands on a
    // hidden process.
    pub fn filtered(&self) -> Vec<&Row> {
        self.rows
            .iter()
            .filter(|r| self.filter.keep(r, &self.flagged) && self.query_matches(r.name()))
            .collect()
    }

    // Resolved by pid, not index: the sort order changes under the selection on
    // every refresh, which is why `State` stores `selected_pid` at all.
    pub fn selected_row(&self) -> Option<&Row> {
        self.rows.iter().find(|r| r.pid == self.selected_pid)
    }
}
