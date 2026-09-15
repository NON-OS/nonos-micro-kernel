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

use crate::store::Store;

#[test]
fn journal_lists_newest_first() {
    let mut s = Store::new();
    s.journal_touch("/a");
    s.journal_touch("/b");
    s.journal_touch("/c");
    let got: alloc::vec::Vec<&str> = s.journal_list(10).into_iter().map(|(_, p)| p).collect();
    assert_eq!(got, alloc::vec!["/c", "/b", "/a"]);
}

#[test]
fn journal_dedupes_and_promotes_a_repeat_touch() {
    let mut s = Store::new();
    s.journal_touch("/a");
    s.journal_touch("/b");
    s.journal_touch("/a");
    let got: alloc::vec::Vec<&str> = s.journal_list(10).into_iter().map(|(_, p)| p).collect();
    assert_eq!(got, alloc::vec!["/a", "/b"]);
}

#[test]
fn journal_honours_the_requested_max() {
    let mut s = Store::new();
    s.journal_touch("/a");
    s.journal_touch("/b");
    assert_eq!(s.journal_list(1).len(), 1);
}

#[test]
fn journal_evicts_the_oldest_at_capacity() {
    let mut s = Store::new();
    for i in 0..300 {
        s.journal_touch(&alloc::format!("/f{i}"));
    }
    let all = s.journal_list(1000);
    assert_eq!(all.len(), 256);
    assert_eq!(all[0].1, "/f299");
    assert!(!all.iter().any(|(_, p)| *p == "/f0"));
}
