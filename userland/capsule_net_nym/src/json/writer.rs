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

use super::byte_array::push_bytes;
use super::escape::escape_into;
use super::number::push_u64;
use alloc::string::String;

/// A tagged request carrying a protocol version and a byte payload, which is
/// the shape every handshake frame this capsule sends happens to have.
pub fn tagged_bytes_request(tag: &str, version: u64, data: &[u8]) -> String {
    let mut out = String::new();
    out.push_str("{\"type\":\"");
    escape_into(&mut out, tag);
    out.push_str("\",\"protocolVersion\":");
    push_u64(&mut out, version);
    out.push_str(",\"data\":");
    push_bytes(&mut out, data);
    out.push('}');
    out
}
