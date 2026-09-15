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

use crate::files_fixture::{put, SEARCH_CASE, SEARCH_CONTENT, SEARCH_NAMES};
use crate::store::Store;

#[test]
fn search_matches_names() {
    let mut s = Store::new();
    put(&mut s, "/notes.txt", b"nothing here");
    put(&mut s, "/other.bin", b"nope");
    let hits = s.search("notes", SEARCH_NAMES, 512);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0], (0, 0, "/notes.txt"));
}

#[test]
fn search_matches_content_with_a_line_number() {
    let mut s = Store::new();
    put(&mut s, "/a.txt", b"alpha\nbeta\ngamma");
    let hits = s.search("beta", SEARCH_CONTENT, 512);
    assert_eq!(hits, alloc::vec![(1, 2, "/a.txt")]);
}

#[test]
fn search_is_case_insensitive_unless_asked() {
    let mut s = Store::new();
    put(&mut s, "/Alpha.txt", b"x");
    assert_eq!(s.search("alpha", SEARCH_NAMES, 512).len(), 1);
    assert_eq!(s.search("alpha", SEARCH_NAMES | SEARCH_CASE, 512).len(), 0);
}

#[test]
fn search_orders_name_hits_before_content_hits() {
    let mut s = Store::new();
    put(&mut s, "/zebra.txt", b"nothing");
    put(&mut s, "/aaa.txt", b"zebra");
    let hits = s.search("zebra", SEARCH_NAMES | SEARCH_CONTENT, 512);
    assert_eq!(hits[0].2, "/zebra.txt");
    assert_eq!(hits[0].0, 0);
    assert_eq!(hits[1].2, "/aaa.txt");
    assert_eq!(hits[1].0, 1);
}

#[test]
fn search_skips_binary_content() {
    let mut s = Store::new();
    put(&mut s, "/b.bin", b"\x00\x01needle\x02");
    assert!(s.search("needle", SEARCH_CONTENT, 512).is_empty());
}

#[test]
fn search_matches_directory_names_with_their_own_kind() {
    let mut s = Store::new();
    s.mkdir("/reports").unwrap();
    put(&mut s, "/reports/q1.txt", b"x");
    let hits = s.search("report", SEARCH_NAMES, 512);
    assert_eq!(
        hits,
        alloc::vec![(2, 0, "/reports"), (0, 0, "/reports/q1.txt")]
    );
}

#[test]
fn search_never_reads_content_out_of_a_directory() {
    let mut s = Store::new();
    s.mkdir("/needle").unwrap();
    assert!(s.search("needle", SEARCH_CONTENT, 512).is_empty());
}
