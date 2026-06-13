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

pub fn request_broadcast(raw_tx: &[u8], id: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw_tx.len() * 2 + 96);
    out.extend_from_slice(b"{\"jsonrpc\":\"2.0\",\"method\":\"eth_sendRawTransaction\",\"params\":[\"");
    super::append_hex_bytes::append_hex_bytes(&mut out, raw_tx);
    out.extend_from_slice(b"\"],\"id\":");
    super::append_dec_u64::append_dec_u64(&mut out, id);
    out.extend_from_slice(b"}");
    out
}
