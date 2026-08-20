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

use super::buffer::response_buffer;
use crate::security::tpm::error::TpmError;

/// Copy out only what the response header claims, after checking that claim
/// against both the published buffer and the caller's slice. A part reporting
/// a longer response than it was given room for would otherwise read past the
/// end of the buffer.
pub(super) fn read_response(out: &mut [u8]) -> Result<usize, TpmError> {
    let buffer = response_buffer()?;
    if buffer.size < 10 || out.len() < 10 {
        return Err(TpmError::InvalidResponse);
    }
    let mut header = [0u8; 10];
    for (i, b) in header.iter_mut().enumerate() {
        // SAFETY: eK@nonos.systems - the first ten bytes of the response
        // buffer the part published, whose size was checked above.
        *b = unsafe { core::ptr::read_volatile((buffer.virt as *const u8).add(i)) };
    }
    let size = u32::from_be_bytes([header[2], header[3], header[4], header[5]]) as usize;
    if size < 10 || size > buffer.size || size > out.len() {
        return Err(TpmError::InvalidResponse);
    }
    for (i, slot) in out.iter_mut().enumerate().take(size) {
        // SAFETY: eK@nonos.systems - bounded by `size`, itself bounded by the
        // published buffer size and the caller's slice.
        *slot = unsafe { core::ptr::read_volatile((buffer.virt as *const u8).add(i)) };
    }
    Ok(size)
}
