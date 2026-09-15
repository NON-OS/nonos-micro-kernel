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

use crate::files_fixture::put;
use crate::store::Store;

#[test]
fn dirstat_counts_recursively() {
    let mut s = Store::new();
    s.mkdir("/p").unwrap();
    s.mkdir("/p/sub").unwrap();
    put(&mut s, "/p/a.txt", b"1234");
    put(&mut s, "/p/sub/b.txt", b"123456");
    put(&mut s, "/other.txt", b"xxxxxxxxxx");

    let (files, dirs, bytes, truncated) = s.dirstat("/p", 20_000);
    assert_eq!(files, 2);
    assert_eq!(dirs, 1);
    assert_eq!(bytes, 10);
    assert!(!truncated);
}

#[test]
fn dirstat_excludes_the_prefix_itself() {
    let mut s = Store::new();
    s.mkdir("/p").unwrap();
    let (files, dirs, bytes, _) = s.dirstat("/p", 20_000);
    assert_eq!((files, dirs, bytes), (0, 0, 0));
}

#[test]
fn dirstat_missing_prefix_is_all_zero() {
    let s = Store::new();
    assert_eq!(s.dirstat("/nope", 20_000), (0, 0, 0, false));
}

#[test]
fn dirstat_reports_truncation_at_the_node_cap() {
    let mut s = Store::new();
    s.mkdir("/p").unwrap();
    for i in 0..10 {
        put(&mut s, &alloc::format!("/p/f{i}"), b"z");
    }
    let (files, _, _, truncated) = s.dirstat("/p", 4);
    assert!(truncated);
    assert_eq!(files, 4);
}
