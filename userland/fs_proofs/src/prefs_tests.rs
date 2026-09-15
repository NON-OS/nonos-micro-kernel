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

use crate::fm_logic::prefs::Prefs;
use crate::fm_logic::state::{SortMode, ViewKind};

#[test]
fn prefs_round_trip() {
    let p = Prefs { view: ViewKind::List, sort: SortMode::Size, sidebar_open: false };
    let back = Prefs::from_blob(&p.to_blob());
    assert!(back.view == ViewKind::List);
    assert!(matches!(back.sort, SortMode::Size));
    assert!(!back.sidebar_open);
}

#[test]
fn prefs_fall_back_to_defaults_on_garbage() {
    let back = Prefs::from_blob(b"not a sidecar blob at all");
    assert!(back.view == ViewKind::Grid);
    assert!(matches!(back.sort, SortMode::Name));
    assert!(back.sidebar_open);
}
