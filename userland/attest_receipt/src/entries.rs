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

//! Entry bytes to records.
//!
//! Strict on length. A trailing partial entry means the set was truncated, and
//! dropping the remainder would turn that into a silent verification failure.

use crate::decode::Authority;
use crate::kernel::ENTRY_LEN;

/// One capsule, as the machine recorded it when the capsule passed its gate.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Entry {
    pub pid: u32,
    pub measurement: [u8; 32],
    pub caps: u64,
    pub authority: Authority,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParseError {
    Ragged { len: usize },
}

impl ParseError {
    pub fn describe(&self) -> String {
        match self {
            Self::Ragged { len } => format!(
                "{len} bytes is not a whole number of {ENTRY_LEN} byte entries, \
                 so the set was truncated or was never complete"
            ),
        }
    }
}

/// Splits the byte string into records. Verifies nothing: fold and compare
/// against the signed root before reading a field.
pub fn parse_entries(bytes: &[u8]) -> Result<Vec<Entry>, ParseError> {
    if !bytes.len().is_multiple_of(ENTRY_LEN) {
        return Err(ParseError::Ragged { len: bytes.len() });
    }
    Ok(bytes.chunks_exact(ENTRY_LEN).map(read_one).collect())
}

fn read_one(b: &[u8]) -> Entry {
    let mut measurement = [0u8; 32];
    measurement.copy_from_slice(&b[4..36]);
    let mut pid = [0u8; 4];
    pid.copy_from_slice(&b[..4]);
    let mut caps = [0u8; 8];
    caps.copy_from_slice(&b[36..44]);
    Entry {
        pid: u32::from_be_bytes(pid),
        measurement,
        caps: u64::from_be_bytes(caps),
        authority: Authority::from_byte(b[44]),
    }
}
