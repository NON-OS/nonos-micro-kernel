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

use super::super::{known_interp, InterpreterInfo, MAX_INTERP_PATH_LEN, NONOS_INTERP};

#[test]
fn test_is_nonos_ld() {
    assert!(InterpreterInfo::new(NONOS_INTERP.into()).is_nonos_ld());
    assert!(InterpreterInfo::new("/lib/ld-nonos.so.2".into()).is_nonos_ld());
    assert!(!InterpreterInfo::new("/lib64/ld-linux-x86-64.so.2".into()).is_nonos_ld());
}

#[test]
fn test_is_glibc_ld() {
    assert!(InterpreterInfo::new("/lib64/ld-linux-x86-64.so.2".into()).is_glibc_ld());
    assert!(InterpreterInfo::new("/lib/ld-linux.so.2".into()).is_glibc_ld());
    assert!(!InterpreterInfo::new(NONOS_INTERP.into()).is_glibc_ld());
}

#[test]
fn test_is_musl_ld() {
    assert!(InterpreterInfo::new("/lib/ld-musl-x86_64.so.1".into()).is_musl_ld());
    assert!(InterpreterInfo::new("/lib/ld-musl-aarch64.so.1".into()).is_musl_ld());
    assert!(!InterpreterInfo::new(NONOS_INTERP.into()).is_musl_ld());
}

#[test]
fn test_is_foreign() {
    assert!(InterpreterInfo::new("/lib64/ld-linux-x86-64.so.2".into()).is_foreign());
    assert!(InterpreterInfo::new("/lib/ld-musl-x86_64.so.1".into()).is_foreign());
    assert!(!InterpreterInfo::new(NONOS_INTERP.into()).is_foreign());
}

#[test]
fn test_constants() {
    assert_eq!(NONOS_INTERP, "/lib/ld-nonos.so.1");
    assert_eq!(MAX_INTERP_PATH_LEN, 4096);
    assert_eq!(known_interp::GLIBC_LD, "ld-linux");
    assert_eq!(known_interp::MUSL_LD, "ld-musl");
    assert_eq!(known_interp::GENERIC_LD, "ld.so");
}
