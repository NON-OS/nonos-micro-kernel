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

/// Where a palette row came from. The tag is drawn in the row's gutter so a
/// reader can tell a remembered command from a builtin at a glance.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Command,
    History,
    Session,
    Project,
    Action,
}

impl Kind {
    pub fn tag(self) -> &'static str {
        match self {
            Kind::Command => "cmd",
            Kind::History => "run",
            Kind::Session => "tab",
            Kind::Project => "dir",
            Kind::Action => "act",
        }
    }
}

/// What accepting a row does. Carries no borrow, so the caller can resolve a
/// selection and then drop the index before touching the window it describes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    Run,
    Session(u16),
    Project(u16),
    NewSession,
    ToggleMonitor,
    ChangeTheme,
}

#[derive(Clone, Copy)]
pub struct Entry<'a> {
    pub kind: Kind,
    pub label: &'a str,
    pub hint: &'a str,
    pub action: Action,
}

impl Default for Entry<'_> {
    fn default() -> Self {
        Self { kind: Kind::Command, label: "", hint: "", action: Action::Run }
    }
}
