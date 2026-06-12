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

use alloc::string::ToString;

use super::super::InterpreterInfo;

#[test]
fn test_interpreter_info_new() {
    let info = InterpreterInfo::new("/lib64/ld-linux-x86-64.so.2".into());
    assert_eq!(info.path, "/lib64/ld-linux-x86-64.so.2");
}

#[test]
fn test_interpreter_info_as_str() {
    assert_eq!(
        InterpreterInfo::new("/lib64/ld-linux-x86-64.so.2".into()).as_str(),
        "/lib64/ld-linux-x86-64.so.2"
    );
}

#[test]
fn test_filename() {
    assert_eq!(
        InterpreterInfo::new("/lib64/ld-linux-x86-64.so.2".into()).filename(),
        "ld-linux-x86-64.so.2"
    );
    assert_eq!(InterpreterInfo::new("ld.so".into()).filename(), "ld.so");
}

#[test]
fn test_is_absolute() {
    assert!(InterpreterInfo::new("/lib64/ld-linux-x86-64.so.2".into()).is_absolute());
    assert!(!InterpreterInfo::new("ld.so".into()).is_absolute());
}

#[test]
fn test_directory() {
    assert_eq!(InterpreterInfo::new("/lib64/ld-linux-x86-64.so.2".into()).directory(), "/lib64");
    assert_eq!(InterpreterInfo::new("/a/b/c/ld.so".into()).directory(), "/a/b/c");
    assert_eq!(InterpreterInfo::new("ld.so".into()).directory(), "");
}

#[test]
fn test_default() {
    assert!(InterpreterInfo::default().path.is_empty());
}

#[test]
fn test_from_string() {
    let info: InterpreterInfo = "/lib64/ld-linux-x86-64.so.2".to_string().into();
    assert_eq!(info.path, "/lib64/ld-linux-x86-64.so.2");
}

#[test]
fn test_from_str() {
    let info: InterpreterInfo = "/lib64/ld-linux-x86-64.so.2".into();
    assert_eq!(info.path, "/lib64/ld-linux-x86-64.so.2");
}
