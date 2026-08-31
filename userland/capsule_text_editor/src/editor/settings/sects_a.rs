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

//! Row tables for Editing, Auto Save and Language. Every switch here is backed
//! by a persisted bit; the dropdowns have no popup behind them, so they are
//! listed as `Drop` and the painter draws them from the dimmed style.

use super::sect::{Ctl, Section};

pub(super) const EDITING: Section = Section {
    head: "Editing",
    rows: &[
        ("Word wrap", Ctl::Toggle(0)),
        ("Show invisible characters", Ctl::Toggle(1)),
        ("Smart quotes", Ctl::Toggle(2)),
        ("Auto-capitalise sentences", Ctl::Toggle(3)),
        ("Highlight the current line", Ctl::Toggle(4)),
        ("Tab width", Ctl::Drop("4 spaces")),
    ],
};

pub(super) const AUTO_SAVE: Section = Section {
    head: "Auto Save",
    rows: &[
        ("Auto save documents", Ctl::Toggle(0)),
        ("Save when the window loses focus", Ctl::Toggle(1)),
        ("Keep version history", Ctl::Toggle(2)),
        ("Save interval", Ctl::Drop("Every 30 seconds")),
        ("Version retention", Ctl::Drop("30 days")),
    ],
};

pub(super) const LANGUAGE: Section = Section {
    head: "Language",
    rows: &[
        ("Detect document language", Ctl::Toggle(0)),
        ("Right-to-left layout", Ctl::Toggle(1)),
        ("Display language", Ctl::Drop("English (US)")),
        ("Document language", Ctl::Drop("English (US)")),
    ],
};
