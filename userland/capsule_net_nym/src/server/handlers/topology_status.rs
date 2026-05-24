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

use crate::protocol::{E_OK, OP_TOPOLOGY_STATUS};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::topology;

const LEN: u32 = 28;

pub fn handle(pid: u32, req: &Request, tx: &mut [u8]) {
    let off = 20usize;
    tx[off..off + 4].copy_from_slice(&status_code().to_le_bytes());
    match topology::meta() {
        Some(meta) => write_meta(&mut tx[off + 4..off + LEN as usize], meta),
        None => tx[off + 4..off + LEN as usize].fill(0),
    }
    respond(pid, OP_TOPOLOGY_STATUS, E_OK, req.request_id, LEN, tx);
}

fn write_meta(out: &mut [u8], meta: topology::DirectoryMeta) {
    out[0..8].copy_from_slice(&meta.epoch.to_le_bytes());
    out[8..16].copy_from_slice(&meta.not_before_ms.to_le_bytes());
    out[16..24].copy_from_slice(&meta.not_after_ms.to_le_bytes());
}

fn status_code() -> u32 {
    match topology::status() {
        topology::TopologyStatus::Missing => 0,
        topology::TopologyStatus::Ready => 1,
        topology::TopologyStatus::Expired => 2,
        topology::TopologyStatus::Clock => 3,
        topology::TopologyStatus::UntrustedAuthority => 4,
    }
}
