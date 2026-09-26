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


//! Turning a guest's path into a store key.

use crate::resolve::absolute;

fn at(cwd: &str, path: &str) -> String {
    String::from_utf8(absolute(cwd.as_bytes(), path.as_bytes())).unwrap()
}

#[test]
fn an_absolute_path_ignores_the_working_directory() {
    assert_eq!(at("/home", "/etc/passwd"), "/etc/passwd");
}

#[test]
fn a_relative_path_hangs_off_the_working_directory() {
    assert_eq!(at("/home", "lib/libc.so"), "/home/lib/libc.so");
}

#[test]
fn dot_components_disappear() {
    assert_eq!(at("/", "./a/./b"), "/a/b");
    assert_eq!(at("/a", "b/../c"), "/a/c");
}

#[test]
fn dot_dot_past_the_root_stops_at_the_root() {
    assert_eq!(at("/", "../../../etc"), "/etc");
    assert_eq!(at("/a", "../../.."), "/");
}

#[test]
fn repeated_separators_collapse() {
    assert_eq!(at("/", "//lib///libc.so"), "/lib/libc.so");
}

#[test]
fn the_root_itself_survives() {
    assert_eq!(at("/", "/"), "/");
    assert_eq!(at("/", ""), "/");
}
