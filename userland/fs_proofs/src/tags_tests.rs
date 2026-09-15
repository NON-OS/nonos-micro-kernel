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

use crate::fm_logic::tags::TagMap;

#[test]
fn tags_add_and_query() {
    let mut m = TagMap::default();
    assert!(m.add("/a.txt", "Work"));
    assert!(m.add("/a.txt", "urgent"));
    assert_eq!(m.tags_for("/a.txt"), alloc::vec!["work", "urgent"]);
    assert_eq!(m.paths_with("work"), alloc::vec!["/a.txt"]);
}

#[test]
fn tags_reject_invalid_names() {
    let mut m = TagMap::default();
    assert!(!m.add("/a", ""));
    assert!(!m.add("/a", "has space"));
    assert!(!m.add("/a", "waaaaaaaaaaaaaaaaaaaaaaaaay-too-long"));
    assert!(m.tags_for("/a").is_empty());
}

#[test]
fn tags_are_capped_per_path() {
    let mut m = TagMap::default();
    for i in 0..8 {
        assert!(m.add("/a", &alloc::format!("t{i}")));
    }
    assert!(!m.add("/a", "t8"));
    assert_eq!(m.tags_for("/a").len(), 8);
}

#[test]
fn tags_add_is_idempotent() {
    let mut m = TagMap::default();
    m.add("/a", "work");
    m.add("/a", "work");
    assert_eq!(m.tags_for("/a").len(), 1);
}

#[test]
fn tags_remove_drops_the_entry_when_empty() {
    let mut m = TagMap::default();
    m.add("/a", "work");
    m.remove("/a", "work");
    assert!(m.tags_for("/a").is_empty());
    assert!(m.paths_with("work").is_empty());
}
