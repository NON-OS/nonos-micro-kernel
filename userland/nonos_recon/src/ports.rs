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

//! Parsing a port specification like `22,80,443` or `1-1024` or `1-100,443`.
//!
//! The result is sorted and free of duplicates, so overlapping ranges probe a
//! port once and the report reads in order. The count is capped so one scan
//! cannot be pointed at an unbounded amount of work: a spec that would expand
//! past the cap is refused rather than silently truncated, since a truncated
//! scan that looked complete would be worse than an error.

extern crate alloc;

use alloc::vec::Vec;

/// Most ports one invocation may probe. The full range fits, and a request for
/// more than the whole range is by definition malformed.
pub const MAX_PORTS: usize = 65_535;

/// Why a port specification is rejected.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PortError {
    /// A field was empty or held a non-digit.
    Malformed,
    /// A port was 0 or above 65535.
    OutOfRange,
    /// A range's end was below its start.
    Backwards,
    /// The spec expanded to more than `MAX_PORTS` ports.
    TooMany,
}

/// Parse a comma-separated list of ports and `start-end` ranges into a sorted,
/// deduplicated port list.
pub fn parse_ports(spec: &str) -> Result<Vec<u16>, PortError> {
    let mut ports: Vec<u16> = Vec::new();
    for field in spec.split(',') {
        let field = trim(field);
        if field.is_empty() {
            return Err(PortError::Malformed);
        }
        match split_once(field, b'-') {
            Some((lo, hi)) => {
                let start = parse_port(lo)?;
                let end = parse_port(hi)?;
                if end < start {
                    return Err(PortError::Backwards);
                }
                let mut p = start;
                loop {
                    push_capped(&mut ports, p)?;
                    if p == end {
                        break;
                    }
                    p += 1;
                }
            }
            None => push_capped(&mut ports, parse_port(field)?)?,
        }
    }
    if ports.is_empty() {
        return Err(PortError::Malformed);
    }
    ports.sort_unstable();
    ports.dedup();
    Ok(ports)
}

/// One port: 1 to 5 digits, value 1 to 65535. Surrounding spaces are trimmed
/// so a range written with spaces around the dash still parses.
fn parse_port(part: &str) -> Result<u16, PortError> {
    let bytes = trim(part).as_bytes();
    if bytes.is_empty() || bytes.len() > 5 {
        return Err(PortError::Malformed);
    }
    let mut value: u32 = 0;
    for b in bytes {
        if !b.is_ascii_digit() {
            return Err(PortError::Malformed);
        }
        value = value * 10 + (b - b'0') as u32;
    }
    if value == 0 || value > 65_535 {
        return Err(PortError::OutOfRange);
    }
    Ok(value as u16)
}

/// Push a port, refusing once the cap is reached. The cap is checked before the
/// push so the vector never grows past it.
fn push_capped(ports: &mut Vec<u16>, port: u16) -> Result<(), PortError> {
    if ports.len() >= MAX_PORTS {
        return Err(PortError::TooMany);
    }
    ports.push(port);
    Ok(())
}

/// Trim ASCII spaces from both ends without allocating.
fn trim(s: &str) -> &str {
    let b = s.as_bytes();
    let mut start = 0;
    while start < b.len() && b[start] == b' ' {
        start += 1;
    }
    let mut end = b.len();
    while end > start && b[end - 1] == b' ' {
        end -= 1;
    }
    // The slice is on ASCII-space boundaries, so it stays valid UTF-8.
    core::str::from_utf8(&b[start..end]).unwrap_or("")
}

/// Split on the first `sep`, returning the two sides, or `None` if absent.
fn split_once(s: &str, sep: u8) -> Option<(&str, &str)> {
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == sep {
            let left = core::str::from_utf8(&b[..i]).unwrap_or("");
            let right = core::str::from_utf8(&b[i + 1..]).unwrap_or("");
            return Some((left, right));
        }
        i += 1;
    }
    None
}
