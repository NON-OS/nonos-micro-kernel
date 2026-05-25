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

use super::super::ElfError;

#[test]
fn test_error_as_str() {
    assert_eq!(ElfError::InvalidMagic.as_str(), "Invalid ELF magic number");
    assert_eq!(ElfError::Other("custom error").as_str(), "custom error");
}

#[test]
fn test_error_display() {
    assert!(alloc::format!("{}", ElfError::InvalidMagic).contains("magic"));
    assert!(alloc::format!("{}", ElfError::UnsupportedRelocation(42)).contains("42"));
}

#[test]
fn test_error_from_str() {
    let error: ElfError = "test error".into();
    assert_eq!(error, ElfError::Other("test error"));
}

#[test]
fn test_error_equality() {
    assert_eq!(ElfError::InvalidMagic, ElfError::InvalidMagic);
    assert_ne!(ElfError::InvalidMagic, ElfError::InvalidClass);
    assert_eq!(ElfError::UnsupportedRelocation(5), ElfError::UnsupportedRelocation(5));
    assert_ne!(ElfError::UnsupportedRelocation(5), ElfError::UnsupportedRelocation(6));
}
