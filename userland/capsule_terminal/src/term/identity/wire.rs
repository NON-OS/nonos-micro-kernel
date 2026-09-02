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

use nonos_policy_proto::{Header, E_OK, HDR_LEN, KIND_STR, OP_GET};

/// A policy `OP_GET` is a bare header: the field is the whole question.
pub const REQ_LEN: usize = HDR_LEN;

pub fn request(field: u32, out: &mut [u8; REQ_LEN]) {
    let hdr = Header { op: OP_GET, field, kind: KIND_STR, status: 0, payload_len: 0 };
    hdr.encode(&mut out[..]);
}

/// The string a policy `OP_GET` reply carries, or `None` if the reply is not
/// an answer to the question that was asked. The server echoes op, field and
/// kind back, so all three are checked rather than trusting arrival order.
pub fn decode_str(field: u32, rx: &[u8]) -> Option<&[u8]> {
    if rx.len() < HDR_LEN {
        return None;
    }
    let h = Header::decode(&rx[..HDR_LEN])?;
    if h.op != OP_GET || h.field != field || h.kind != KIND_STR || h.status != E_OK {
        return None;
    }
    let end = HDR_LEN.checked_add(h.payload_len as usize)?;
    if end > rx.len() {
        return None;
    }
    Some(&rx[HDR_LEN..end])
}
