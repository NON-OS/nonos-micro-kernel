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

use super::types::{Container, ENTRY_LEN, HEADER_LEN, MAGIC};

pub fn encode_unsigned(c: &Container) -> Vec<u8> {
    let table_len = c.sections.len() * ENTRY_LEN;
    let mut offset = (HEADER_LEN + table_len) as u32;
    let mut table = Vec::with_capacity(table_len);
    let mut payload = Vec::new();
    for s in &c.sections {
        let len = s.bytes.len() as u32;
        table.extend_from_slice(&(s.kind as u16).to_be_bytes());
        table.extend_from_slice(&0u16.to_be_bytes());
        table.extend_from_slice(&offset.to_be_bytes());
        table.extend_from_slice(&len.to_be_bytes());
        table.extend_from_slice(&0u32.to_be_bytes());
        payload.extend_from_slice(&s.bytes);
        offset += len;
    }
    let mut out = Vec::with_capacity(HEADER_LEN + table.len() + payload.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&(c.sections.len() as u16).to_be_bytes());
    out.extend_from_slice(&table);
    out.extend_from_slice(&payload);
    out
}
