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

use super::constants::{MAGIC, TOPOLOGY_STATUS};
use crate::clients::envelope::call;

#[derive(Clone, Copy)]
pub struct TopologyStatus {
    pub status: u32,
    pub epoch: u64,
    pub not_before_ms: u64,
    pub not_after_ms: u64,
}

pub fn topology_status(port: u32) -> Result<TopologyStatus, u16> {
    let mut out = [0u8; 28];
    if call(port, MAGIC, TOPOLOGY_STATUS, &[], &mut out)? != out.len() {
        return Err(4);
    }
    Ok(TopologyStatus {
        status: u32::from_le_bytes([out[0], out[1], out[2], out[3]]),
        epoch: u64_at(&out, 4),
        not_before_ms: u64_at(&out, 12),
        not_after_ms: u64_at(&out, 20),
    })
}

fn u64_at(buf: &[u8], off: usize) -> u64 {
    u64::from_le_bytes([
        buf[off],
        buf[off + 1],
        buf[off + 2],
        buf[off + 3],
        buf[off + 4],
        buf[off + 5],
        buf[off + 6],
        buf[off + 7],
    ])
}
