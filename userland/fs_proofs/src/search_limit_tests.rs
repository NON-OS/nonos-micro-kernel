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

use crate::files_fixture::{put, SEARCH_NAMES};
use crate::store::Store;

#[test]
fn search_honours_max_hits() {
    let mut s = Store::new();
    for i in 0..10 {
        put(&mut s, &alloc::format!("/needle{i}.txt"), b"x");
    }
    assert_eq!(s.search("needle", SEARCH_NAMES, 3).len(), 3);
}

#[test]
fn search_with_no_axis_flags_returns_nothing() {
    let mut s = Store::new();
    put(&mut s, "/a.txt", b"a");
    assert!(s.search("a", 0, 512).is_empty());
}
