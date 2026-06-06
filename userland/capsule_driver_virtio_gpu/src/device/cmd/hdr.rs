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
pub const HDR_LEN: usize = 24;
pub const RESP_HDR_LEN: usize = 24;
#[derive(Clone, Copy)]
pub struct Hdr {
    pub type_: u32,
    pub flags: u32,
    pub fence_id: u64,
    pub ctx_id: u32,
}
impl Hdr {
    pub fn new(type_: u32, fence_id: u64) -> Self {
        Self { type_, flags: 0, fence_id, ctx_id: 0 }
    }
    pub fn write(self, out: &mut [u8]) {
        out[0..4].copy_from_slice(&self.type_.to_le_bytes());
        out[4..8].copy_from_slice(&self.flags.to_le_bytes());
        out[8..16].copy_from_slice(&self.fence_id.to_le_bytes());
        out[16..20].copy_from_slice(&self.ctx_id.to_le_bytes());
        out[20..24].fill(0);
    }
    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < HDR_LEN {
            return None;
        }
        Some(Self {
            type_: u32::from_le_bytes(buf[0..4].try_into().ok()?),
            flags: u32::from_le_bytes(buf[4..8].try_into().ok()?),
            fence_id: u64::from_le_bytes(buf[8..16].try_into().ok()?),
            ctx_id: u32::from_le_bytes(buf[16..20].try_into().ok()?),
        })
    }
}
