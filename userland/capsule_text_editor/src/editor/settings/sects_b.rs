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

//! Row tables for Spelling & Grammar, Collaboration and Advanced. Same split as
//! `sects_a`: switches are live and persisted, dropdowns are declarative labels
//! the painter renders dimmed until a popup exists to open.

use super::sect::{Ctl, Section};

pub(super) const SPELLING: Section = Section {
    head: "Spelling & Grammar",
    rows: &[
        ("Check spelling as you type", Ctl::Toggle(0)),
        ("Check grammar", Ctl::Toggle(1)),
        ("Ignore words in UPPERCASE", Ctl::Toggle(2)),
        ("Ignore words containing numbers", Ctl::Toggle(3)),
        ("Dictionary", Ctl::Drop("English (US)")),
    ],
};

pub(super) const COLLABORATION: Section = Section {
    head: "Collaboration",
    rows: &[
        ("Show collaborator cursors", Ctl::Toggle(0)),
        ("Show presence in the status bar", Ctl::Toggle(1)),
        ("Notify me about new comments", Ctl::Toggle(2)),
        ("Suggest edits instead of editing", Ctl::Toggle(3)),
        ("Default share access", Ctl::Drop("View only")),
    ],
};

pub(super) const ADVANCED: Section = Section {
    head: "Advanced",
    rows: &[
        ("Hardware accelerated rendering", Ctl::Toggle(0)),
        ("Restore the session on relaunch", Ctl::Toggle(1)),
        ("Verbose capsule logging", Ctl::Toggle(2)),
        ("Show the performance overlay", Ctl::Toggle(3)),
        ("Crash report detail", Ctl::Drop("Minimal")),
    ],
};
