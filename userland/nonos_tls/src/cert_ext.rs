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

// Extensions live in [3] EXPLICIT at the end of TBSCertificate, after the
// optional issuerUniqueID [1] and subjectUniqueID [2]. No extensions means a
// v1 or v2 certificate, which cannot carry the constraints a CA is recognised
// by, so the absent case is None rather than an empty list.
pub fn extension_value<'a>(cert: &'a [u8], oid: &[u8]) -> Option<&'a [u8]> {
    let (start, list_end) = extension_list(cert)?;
    let mut cur = start;
    while cur < list_end {
        let (tag, inner, next) = super::der_tlv::der_tlv(cert, cur)?;
        if tag != 0x30 {
            return None;
        }
        if let Some(value) = super::cert_ext_entry::match_extension(cert, inner, next, oid) {
            return Some(value);
        }
        cur = next;
    }
    None
}

fn extension_list(cert: &[u8]) -> Option<(usize, usize)> {
    let (tag, val, end) = super::der_tlv::der_tlv(cert, 0)?;
    if tag != 0x30 || end != cert.len() {
        return None;
    }
    let (tag, tbs, tbs_end) = super::der_tlv::der_tlv(cert, val)?;
    if tag != 0x30 {
        return None;
    }
    let mut pos = tbs;
    if cert.get(pos).copied() == Some(0xa0) {
        pos = super::der_tlv::der_tlv(cert, pos)?.2;
    }
    // serial, signature, issuer, validity, subject, subjectPublicKeyInfo.
    for _ in 0..6 {
        pos = super::der_tlv::der_tlv(cert, pos)?.2;
    }
    while matches!(cert.get(pos).copied(), Some(0xa1) | Some(0xa2)) {
        pos = super::der_tlv::der_tlv(cert, pos)?.2;
    }
    if cert.get(pos).copied() != Some(0xa3) || pos >= tbs_end {
        return None;
    }
    let (_, outer, _) = super::der_tlv::der_tlv(cert, pos)?;
    let (tag, start, list_end) = super::der_tlv::der_tlv(cert, outer)?;
    if tag != 0x30 || list_end > tbs_end {
        return None;
    }
    Some((start, list_end))
}
