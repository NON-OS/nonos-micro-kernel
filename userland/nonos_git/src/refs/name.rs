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

//! What a branch may be called.
//!
//! A ref name becomes a path under the git directory, so the rules here are
//! load-bearing rather than cosmetic: a name holding `..` or a leading `/`
//! would let a ref write outside `refs/`.

/// Whether `name` is a ref name we will create or follow.
///
/// The rules are git's, restricted to what this implementation needs: no empty
/// component, no `..`, no leading or trailing `/`, no ASCII control or space,
/// none of the characters git reserves for its revision syntax, and no `.lock`
/// suffix, which would collide with the lock files git writes.
pub fn is_valid_ref_name(name: &str) -> bool {
    if name.is_empty() || name.starts_with('/') || name.ends_with('/') {
        return false;
    }
    if name.ends_with(".lock") || name.ends_with('.') {
        return false;
    }
    if name.contains("..") || name.contains("//") || name.contains("@{") {
        return false;
    }
    for c in name.chars() {
        if c.is_ascii_control() || c == ' ' {
            return false;
        }
        if matches!(c, '~' | '^' | ':' | '?' | '*' | '[' | '\\') {
            return false;
        }
    }
    // A component may not begin with a dot, which would make a hidden file, nor
    // be empty, which `//` already covers.
    name.split('/').all(|part| !part.is_empty() && !part.starts_with('.'))
}
