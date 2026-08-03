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
//! Reading the encrypted response records.

extern crate alloc;

use alloc::vec::Vec;

use super::traits::{Io, SessionError};

/// The request asks the server to close after answering, so the response ends
/// when the socket goes quiet. Quiet has to be counted rather than waited on,
/// because the socket layer returns zero on a timeout rather than blocking.
const QUIET_READS: u32 = 60;

pub(super) fn read_response<S: Io>(io: &mut S, limit: usize) -> Result<Vec<u8>, SessionError> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 4096];
    let mut quiet = 0u32;
    while quiet < QUIET_READS {
        let n = io.read(&mut chunk)?;
        if n == 0 {
            quiet += 1;
            continue;
        }
        quiet = 0;
        if buf.len() + n > limit {
            return Err(SessionError::TooLarge);
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    Ok(buf)
}
