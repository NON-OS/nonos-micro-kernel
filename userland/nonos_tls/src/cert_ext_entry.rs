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

// Extension ::= SEQUENCE { extnID OID, critical BOOLEAN DEFAULT FALSE,
// extnValue OCTET STRING }. The default makes the flag commonly absent, so its
// presence is tested rather than assumed.
pub fn match_extension<'a>(
    cert: &'a [u8],
    inner: usize,
    end: usize,
    oid: &[u8],
) -> Option<&'a [u8]> {
    let (tag, oid_val, oid_end) = super::der_tlv::der_tlv(cert, inner)?;
    if tag != 0x06 || oid_end > end || &cert[oid_val..oid_end] != oid {
        return None;
    }
    let mut pos = oid_end;
    if cert.get(pos).copied() == Some(0x01) {
        pos = super::der_tlv::der_tlv(cert, pos)?.2;
    }
    let (tag, val, val_end) = super::der_tlv::der_tlv(cert, pos)?;
    if tag != 0x04 || val_end > end {
        return None;
    }
    Some(&cert[val..val_end])
}
