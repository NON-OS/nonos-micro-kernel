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

use super::super::error::CapsuleFsError;
use super::super::protocol::{encode_open, OPEN_FLAG_CREATE, OPEN_FLAG_TRUNCATE};
use super::super::state;
use super::seq::next;
use super::transport::round_trip;

pub struct OpenResult {
    pub remote_handle: u64,
    pub generation: u64,
}

pub fn open(path: &str, posix_flags: i32) -> Result<OpenResult, CapsuleFsError> {
    let generation = state::current_generation();
    let mut wire = 0u32;
    if posix_flags & crate::fs::fd::O_CREAT != 0 {
        wire |= OPEN_FLAG_CREATE;
    }
    if posix_flags & crate::fs::fd::O_TRUNC != 0 {
        wire |= OPEN_FLAG_TRUNCATE;
    }
    let seq = next();
    let resp = round_trip(seq, encode_open(seq, wire, path))?;
    if resp.status < 0 {
        return Err(status_to_error(resp.status));
    }
    if resp.payload.len() != 8 {
        return Err(CapsuleFsError::TransportFailure);
    }
    let h = u64::from_le_bytes([
        resp.payload[0],
        resp.payload[1],
        resp.payload[2],
        resp.payload[3],
        resp.payload[4],
        resp.payload[5],
        resp.payload[6],
        resp.payload[7],
    ]);
    Ok(OpenResult { remote_handle: h, generation })
}

fn status_to_error(status: i32) -> CapsuleFsError {
    match status {
        -2 => CapsuleFsError::NotFound,
        -22 => CapsuleFsError::InvalidArgument,
        -24 => CapsuleFsError::TooManyFiles,
        -5 => CapsuleFsError::Io,
        _ => CapsuleFsError::Io,
    }
}
