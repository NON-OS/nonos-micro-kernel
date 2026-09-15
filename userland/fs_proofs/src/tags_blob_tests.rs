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

use crate::fm_logic::{tags::TagMap, tags_reconcile::reconcile};

#[test]
fn tags_round_trip_through_the_sidecar_blob() {
    let mut m = TagMap::default();
    m.add("/a", "work");
    m.add("/b", "home");
    let back = TagMap::from_blob(&m.to_blob());
    assert_eq!(back.tags_for("/a"), alloc::vec!["work"]);
    assert_eq!(back.tags_for("/b"), alloc::vec!["home"]);
}

#[test]
fn reconcile_drops_vanished_paths_only_under_the_prefix() {
    let mut m = TagMap::default();
    m.add("/p/gone.txt", "work");
    m.add("/p/here.txt", "work");
    m.add("/elsewhere.txt", "work");
    let live = alloc::vec![alloc::string::String::from("/p/here.txt")];
    reconcile(&mut m, "/p/", &live);
    assert!(m.tags_for("/p/gone.txt").is_empty());
    assert_eq!(m.tags_for("/p/here.txt"), alloc::vec!["work"]);
    assert_eq!(m.tags_for("/elsewhere.txt"), alloc::vec!["work"]);
}

#[test]
fn from_blob_collapses_duplicate_path_records() {
    let mut raw = alloc::vec::Vec::new();
    for _ in 0..4 {
        raw.push((
            alloc::string::String::from("/a/b"),
            alloc::string::String::from("work"),
        ));
    }
    let m = TagMap::from_blob(&crate::sidecar::encode(&raw));
    assert_eq!(m.tagged_paths().len(), 1);
    assert_eq!(m.tags_for("/a/b"), alloc::vec!["work"]);
}
