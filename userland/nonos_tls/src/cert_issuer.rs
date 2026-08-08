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

// Return the issuer Name as raw DER (the SEQUENCE including tag and length),
// suitable for byte-equality against a trusted root's SubjectDN. The issuer is
// the field following the optional version, serialNumber and signature AlgId in
// the TBSCertificate.
pub fn cert_issuer(cert: &[u8]) -> Option<&[u8]> {
    let (tag, val, _end) = super::der_tlv::der_tlv(cert, 0)?;
    if tag != 0x30 {
        return None;
    }
    let (tag, tbs, _tbs_end) = super::der_tlv::der_tlv(cert, val)?;
    if tag != 0x30 {
        return None;
    }
    let mut pos = tbs;
    if cert.get(pos).copied() == Some(0xa0) {
        pos = super::der_tlv::der_tlv(cert, pos)?.2;
    }
    // serialNumber
    pos = super::der_tlv::der_tlv(cert, pos)?.2;
    // signature AlgorithmIdentifier
    pos = super::der_tlv::der_tlv(cert, pos)?.2;
    // issuer Name
    let start = pos;
    let (tag, _iv, end) = super::der_tlv::der_tlv(cert, pos)?;
    if tag != 0x30 {
        return None;
    }
    cert.get(start..end)
}
