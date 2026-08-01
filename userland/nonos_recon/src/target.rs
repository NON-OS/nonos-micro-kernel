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

//! The scan target: a dotted-quad IPv4 address.
//!
//! Only literal addresses are accepted here; name resolution is a separate
//! step the capsule does through the DNS service before it reaches the engine.
//! Parsing rejects anything that is not four decimal octets in range, so a
//! malformed target cannot be read as some other address.

/// A resolved IPv4 target.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Target {
    pub octets: [u8; 4],
}

/// Why a string is not a valid IPv4 target.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TargetError {
    /// Not exactly four dot-separated parts.
    Shape,
    /// An octet was empty, non-numeric, or above 255.
    Octet,
}

/// Parse `s` as a dotted-quad IPv4 address.
pub fn parse_target(s: &str) -> Result<Target, TargetError> {
    let bytes = s.as_bytes();
    let mut octets = [0u8; 4];
    let mut idx = 0usize;
    let mut start = 0usize;

    let mut i = 0usize;
    while i <= bytes.len() {
        let at_end = i == bytes.len();
        if at_end || bytes[i] == b'.' {
            if idx >= 4 {
                return Err(TargetError::Shape);
            }
            octets[idx] = parse_octet(&bytes[start..i])?;
            idx += 1;
            start = i + 1;
        }
        i += 1;
    }

    if idx != 4 {
        return Err(TargetError::Shape);
    }
    Ok(Target { octets })
}

/// One octet: 1 to 3 digits, value 0 to 255, no leading sign or space.
fn parse_octet(part: &[u8]) -> Result<u8, TargetError> {
    if part.is_empty() || part.len() > 3 {
        return Err(TargetError::Octet);
    }
    let mut value: u16 = 0;
    for b in part {
        if !b.is_ascii_digit() {
            return Err(TargetError::Octet);
        }
        value = value * 10 + (b - b'0') as u16;
    }
    if value > 255 {
        return Err(TargetError::Octet);
    }
    Ok(value as u8)
}
