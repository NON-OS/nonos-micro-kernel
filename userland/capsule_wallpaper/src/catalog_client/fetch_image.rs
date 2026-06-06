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

use alloc::vec::Vec;

use super::fetch_chunk::fetch_chunk;
use super::fetch_size::fetch_size;
use super::proto::{CHUNK_MAX, HDR_LEN, IPC_PAYLOAD_MAX};

const MAX_IMAGE_BYTES: u32 = 2_000_000;
const MAX_CHUNKS: u32 = (MAX_IMAGE_BYTES / CHUNK_MAX as u32) + 2;

pub fn fetch_image(catalog_port: u32, index: u32) -> Option<Vec<u8>> {
    let size = fetch_size(catalog_port, index)?;
    if size == 0 || size > MAX_IMAGE_BYTES {
        return None;
    }
    let mut out: Vec<u8> = Vec::with_capacity(size as usize);
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    let mut offset: u32 = 0;
    let mut iterations: u32 = 0;
    while offset < size {
        iterations += 1;
        if iterations > MAX_CHUNKS {
            return None;
        }
        let payload_len = fetch_chunk(catalog_port, index, offset, &mut buf)?;
        let body_end = HDR_LEN + payload_len as usize;
        let new_offset = offset.checked_add(payload_len)?;
        if new_offset > size {
            return None;
        }
        out.extend_from_slice(&buf[HDR_LEN..body_end]);
        offset = new_offset;
    }
    if out.len() as u32 != size {
        return None;
    }
    Some(out)
}
