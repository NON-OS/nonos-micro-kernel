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

use alloc::string::String;
use alloc::vec::Vec;

use crate::fm_logic::{build_entries, ext, kind_of, Entry, Kind};

fn paths(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| String::from(*s)).collect()
}

#[test]
fn build_entries_separates_files_and_dirs() {
    let p = paths(&["/a.txt", "/dir/x", "/b.txt"]);
    let e = build_entries("/", &p);
    let file = e.iter().find(|e| e.label == "a.txt").unwrap();
    assert!(!file.is_dir);
    assert_eq!(file.full_path, "/a.txt");
    let dir = e.iter().find(|e| e.label == "dir/").unwrap();
    assert!(dir.is_dir);
    assert_eq!(dir.full_path, "/dir/");
}

#[test]
fn build_entries_deduplicates_directories() {
    // A directory with several children appears exactly once.
    let p = paths(&["/dir/a", "/dir/b", "/dir/c"]);
    let e = build_entries("/", &p);
    assert_eq!(e.iter().filter(|e| e.label == "dir/").count(), 1);
    assert_eq!(e.len(), 1);
}

#[test]
fn build_entries_lists_only_the_current_level() {
    let p = paths(&["/sub/a", "/sub/deep/b", "/top.txt"]);
    let e = build_entries("/", &p);
    let labels: Vec<&str> = e.iter().map(|e| e.label.as_str()).collect();
    assert!(labels.contains(&"sub/"));
    assert!(labels.contains(&"top.txt"));
    // "deep" is two levels down and must not surface at the root.
    assert!(!labels.iter().any(|l| l.starts_with("deep")));
}

#[test]
fn build_entries_respects_prefix() {
    let p = paths(&["/sub/a.txt", "/sub/nested/b", "/other"]);
    let e = build_entries("/sub/", &p);
    let labels: Vec<&str> = e.iter().map(|e| e.label.as_str()).collect();
    assert!(labels.contains(&"a.txt"));
    assert!(labels.contains(&"nested/"));
    // "/other" is outside the prefix.
    assert!(!labels.contains(&"other"));
}

#[test]
fn ext_reads_the_final_component() {
    assert_eq!(ext("file.txt"), "txt");
    assert_eq!(ext("archive.tar.gz"), "gz");
    assert_eq!(ext("noext"), "");
    assert_eq!(ext("dir.d/"), "d");
}

#[test]
fn ext_treats_dotfiles_as_extensionless() {
    assert_eq!(ext(".gitignore"), "");
    assert_eq!(ext(".env"), "");
    assert_eq!(ext("trailing."), "");
}

#[test]
fn kind_of_classifies_by_extension_and_dir() {
    assert_eq!(kind_of(&entry("src.rs", false)), Kind::Code);
    assert_eq!(kind_of(&entry("logo.svg", false)), Kind::Image);
    assert_eq!(kind_of(&entry("photo.jpeg", false)), Kind::Image);
    assert_eq!(kind_of(&entry("notes.txt", false)), Kind::Doc);
    assert_eq!(kind_of(&entry("bundle.tar", false)), Kind::Archive);
    assert_eq!(kind_of(&entry("kernel.elf", false)), Kind::Exec);
    assert_eq!(kind_of(&entry("mystery.xyz", false)), Kind::Other);
    assert_eq!(kind_of(&entry("folder", true)), Kind::Dir);
}

fn entry(label: &str, is_dir: bool) -> Entry {
    Entry {
        label: String::from(label),
        full_path: String::from(label),
        is_dir,
        size: None,
        mtime: 0,
        writable: true,
    }
}
