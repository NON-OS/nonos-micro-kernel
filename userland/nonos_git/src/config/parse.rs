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
//! Walking a config file.

extern crate alloc;

use alloc::string::String;

/// Call `found` with every `section`, `subsection`, `name`, `value`.
///
/// A section header is `[core]` or `[remote "origin"]`. Everything else is
/// either a `name = value` line inside the section last seen, a comment, or
/// blank. Lines that fit none of those are skipped: git writes settings this
/// does not model, and refusing the whole file over one of them would make a
/// repository unreadable for no gain.
pub(super) fn walk<F: FnMut(&str, &str, &str, &str)>(text: &str, mut found: F) {
    let mut section = String::new();
    let mut subsection = String::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if let Some(header) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            let (name, sub) = match header.split_once(' ') {
                Some((n, s)) => (n, s.trim().trim_matches('"')),
                None => (header, ""),
            };
            section = String::from(name.trim());
            subsection = String::from(sub);
            continue;
        }
        if let Some((name, value)) = line.split_once('=') {
            found(&section, &subsection, name.trim(), value.trim());
        }
    }
}
