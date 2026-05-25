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

use super::state::NAME_MAX;

const RFC1123_MAX_LABEL: usize = 63;
const RFC1123_MAX_TOTAL: usize = 253;

pub(super) fn host(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Hostname cannot be empty");
    }
    let limit = const_min(NAME_MAX - 1, RFC1123_MAX_TOTAL);
    if name.len() > limit {
        return Err("Hostname too long");
    }
    validate_labels(name)
}

pub(super) fn domain(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Ok(());
    }
    let limit = const_min(NAME_MAX - 1, RFC1123_MAX_TOTAL);
    if name.len() > limit {
        return Err("Domainname too long");
    }
    validate_labels(name)
}

fn const_min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

fn validate_labels(name: &str) -> Result<(), &'static str> {
    if name.starts_with('.') || name.ends_with('.') {
        return Err("Invalid label position");
    }
    for label in name.split('.') {
        validate_label(label)?;
    }
    Ok(())
}

fn validate_label(label: &str) -> Result<(), &'static str> {
    if label.is_empty() {
        return Err("Empty DNS label");
    }
    if label.len() > RFC1123_MAX_LABEL {
        return Err("DNS label too long");
    }
    let bytes = label.as_bytes();
    if !bytes[0].is_ascii_alphanumeric() {
        return Err("Label must start alphanumeric");
    }
    if !bytes[bytes.len() - 1].is_ascii_alphanumeric() {
        return Err("Label must end alphanumeric");
    }
    if !bytes.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'-') {
        return Err("Invalid label characters");
    }
    Ok(())
}
