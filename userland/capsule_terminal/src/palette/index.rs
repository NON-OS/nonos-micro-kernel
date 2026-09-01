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

use super::entry::{Action, Entry, Kind};
use super::verbs::VERBS;
use crate::term::prefs::types::Project;
use crate::term::state::State;

/// Rows the index can hold: the verb table, a slice of history, every session
/// and project, and the actions, with room to spare. Fixed so building an
/// index costs no allocation on the query path.
pub const CAP: usize = 48;

pub struct Index<'a> {
    items: [Entry<'a>; CAP],
    len: usize,
}

impl<'a> Index<'a> {
    pub fn build(tabs: &'a [State], active: usize, projects: &'a [Project]) -> Self {
        let mut ix = Self { items: [Entry::default(); CAP], len: 0 };
        for (verb, hint) in VERBS.iter() {
            ix.push(Kind::Command, verb, hint, Action::Run);
        }
        if let Some(s) = tabs.get(active) {
            ix.history(s);
        }
        ix.sessions(tabs);
        for (i, p) in projects.iter().enumerate() {
            ix.push(Kind::Project, p.as_str(), "enter", Action::Project(i as u16));
        }
        ix.push(Kind::Action, "New Session", "another tab", Action::NewSession);
        ix.push(Kind::Action, "Toggle Monitor", "the telemetry rail", Action::ToggleMonitor);
        ix.push(Kind::Action, "Change Theme", "the next palette", Action::ChangeTheme);
        ix
    }

    pub(super) fn push(&mut self, kind: Kind, label: &'a str, hint: &'a str, action: Action) {
        if self.len < CAP && !label.is_empty() {
            self.items[self.len] = Entry { kind, label, hint, action };
            self.len += 1;
        }
    }

    pub fn slice(&self) -> &[Entry<'a>] {
        &self.items[..self.len]
    }
}
