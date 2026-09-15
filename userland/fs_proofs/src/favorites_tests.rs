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

use crate::fm_logic::favorites::Favorites;

#[test]
fn favorites_keep_insertion_order_through_a_round_trip() {
    let mut f = Favorites::default();
    assert!(f.add("/z"));
    assert!(f.add("/a"));
    assert!(f.add("/m"));
    let back = Favorites::from_blob(&f.to_blob());
    assert_eq!(back.list(), alloc::vec!["/z", "/a", "/m"]);
}

#[test]
fn favorites_reject_duplicates_and_cap_at_32() {
    let mut f = Favorites::default();
    assert!(f.add("/a"));
    assert!(!f.add("/a"));
    for i in 0..31 {
        f.add(&alloc::format!("/p{i}"));
    }
    assert_eq!(f.list().len(), 32);
    assert!(!f.add("/overflow"));
}

#[test]
fn favorites_remove_closes_the_gap() {
    let mut f = Favorites::default();
    f.add("/a");
    f.add("/b");
    f.add("/c");
    f.remove("/b");
    assert_eq!(Favorites::from_blob(&f.to_blob()).list(), alloc::vec!["/a", "/c"]);
}
