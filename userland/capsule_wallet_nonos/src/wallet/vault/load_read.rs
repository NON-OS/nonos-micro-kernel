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

//! Reading the sealed blob back off an open descriptor.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::format::BLOB_LEN;
use super::vfs::{call, HDR_LEN, OP_READ};

/// `None` for anything that is not exactly a blob.
///
/// A short read is a refusal rather than a partial buffer. The blob is fixed
/// length by construction, so fewer bytes means a truncated file or a store
/// that answered something else, and either way there is nothing here to
/// hand to the opener. Padding to length would present a record that fails
/// its tag as though the wallet had been tampered with.
pub(super) fn read_exact(fd: u32) -> Option<[u8; BLOB_LEN]> {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(12);
    body.extend_from_slice(&pid.to_le_bytes());
    body.extend_from_slice(&fd.to_le_bytes());
    body.extend_from_slice(&(BLOB_LEN as u32).to_le_bytes());

    let mut rx = vec![0u8; HDR_LEN + 8 + BLOB_LEN];
    let total = call(OP_READ, &body, &mut rx).len()?;
    let data = rx.get(HDR_LEN + 4..total)?;
    if data.len() != BLOB_LEN {
        return None;
    }
    let mut out = [0u8; BLOB_LEN];
    out.copy_from_slice(data);
    Some(out)
}
