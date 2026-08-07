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

pub const MANIFEST: &[u8] = b"manifest-bytes";
pub const ELF: &[u8] = b"\x7fELF-fake-payload";
pub const IDCERT: &[u8] = b"id-cert-bytes";
pub const ZKTR: &[u8] = b"zk-trailer-proof";

pub fn pack(sections: &[(u16, &[u8])]) -> Vec<u8> {
    let table_len = sections.len() * 16;
    let mut offset = (8 + table_len) as u32;
    let mut table = Vec::new();
    let mut payload = Vec::new();
    for (kind, bytes) in sections {
        table.extend_from_slice(&kind.to_be_bytes());
        table.extend_from_slice(&0u16.to_be_bytes());
        table.extend_from_slice(&offset.to_be_bytes());
        table.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
        table.extend_from_slice(&0u32.to_be_bytes());
        payload.extend_from_slice(bytes);
        offset += bytes.len() as u32;
    }
    let mut out = Vec::new();
    out.extend_from_slice(b"NOS1");
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&(sections.len() as u16).to_be_bytes());
    out.extend_from_slice(&table);
    out.extend_from_slice(&payload);
    out
}

pub fn canonical() -> Vec<u8> {
    pack(&[(1, MANIFEST), (2, ELF), (3, IDCERT), (4, ZKTR)])
}

pub fn trailer(sigs: &[(u8, &[u8])]) -> Vec<u8> {
    let mut t = vec![sigs.len() as u8];
    for (tag, sig) in sigs {
        t.push(*tag);
        t.extend_from_slice(&(sig.len() as u16).to_be_bytes());
        t.extend_from_slice(sig);
    }
    t
}
