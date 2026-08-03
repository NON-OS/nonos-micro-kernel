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

// Embedded Mozilla trust anchors, generated from nonos-data/cacert.pem. Each
// record is the CA SubjectDN DER and its SubjectPublicKeyInfo DER, so a chain
// whose top served certificate omits the root can still be anchored by matching
// the top certificate's IssuerDN to a root Subject and verifying its signature.
//
// Layout: [u32 count] then, per record,
//   [u16 subject_len][subject_der][u16 spki_len][spki_der]
static STORE: &[u8] = include_bytes!("store.bin");

fn u16_at(buf: &[u8], off: usize) -> Option<usize> {
    let hi = *buf.get(off + 1)? as usize;
    let lo = *buf.get(off)? as usize;
    Some((hi << 8) | lo)
}

// Return the SPKI DER of a trusted root whose SubjectDN equals `subject`.
pub fn find_spki_by_subject(subject: &[u8]) -> Option<&'static [u8]> {
    if STORE.len() < 4 {
        return None;
    }
    let count = u32::from_le_bytes([STORE[0], STORE[1], STORE[2], STORE[3]]) as usize;
    let mut pos = 4usize;
    let mut i = 0usize;
    while i < count {
        let subj_len = u16_at(STORE, pos)?;
        let subj_start = pos.checked_add(2)?;
        let subj_end = subj_start.checked_add(subj_len)?;
        let subj = STORE.get(subj_start..subj_end)?;
        let spki_len = u16_at(STORE, subj_end)?;
        let spki_start = subj_end.checked_add(2)?;
        let spki_end = spki_start.checked_add(spki_len)?;
        let spki = STORE.get(spki_start..spki_end)?;
        if subj == subject {
            return Some(spki);
        }
        pos = spki_end;
        i += 1;
    }
    None
}
