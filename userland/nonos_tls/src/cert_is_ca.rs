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

// id-ce-basicConstraints, 2.5.29.19.
const OID_BASIC_CONSTRAINTS: [u8; 3] = [0x55, 0x1d, 0x13];
// id-ce-keyUsage, 2.5.29.15.
const OID_KEY_USAGE: [u8; 3] = [0x55, 0x1d, 0x0f];
// keyCertSign is bit 5, counted from the most significant bit of the first
// content byte of the BIT STRING.
const KEY_CERT_SIGN: u8 = 0x04;

/// Whether this certificate is permitted to have issued the one below it.
///
/// Without this a leaf certificate for any domain an attacker controls can be
/// presented as an intermediate and used to sign a certificate for any host,
/// because every signature in such a chain verifies correctly. Absent or
/// unparseable constraints deny.
pub fn cert_is_ca(cert: &[u8]) -> bool {
    let Some(value) = super::cert_ext::extension_value(cert, &OID_BASIC_CONSTRAINTS) else {
        return false;
    };
    if !basic_constraints_ca(value) {
        return false;
    }
    key_usage_allows_signing(cert)
}

// BasicConstraints ::= SEQUENCE { cA BOOLEAN DEFAULT FALSE, ... }. The default
// means an empty sequence is a valid encoding of "not a CA".
fn basic_constraints_ca(value: &[u8]) -> bool {
    let Some((tag, inner, end)) = super::der_tlv::der_tlv(value, 0) else {
        return false;
    };
    if tag != 0x30 || inner >= end {
        return false;
    }
    let Some((tag, val, val_end)) = super::der_tlv::der_tlv(value, inner) else {
        return false;
    };
    tag == 0x01 && val_end == val + 1 && value[val] != 0
}

// KeyUsage is optional. When a certificate carries one it is binding, so a CA
// that did not assert keyCertSign is refused.
fn key_usage_allows_signing(cert: &[u8]) -> bool {
    let Some(value) = super::cert_ext::extension_value(cert, &OID_KEY_USAGE) else {
        return true;
    };
    let Some((tag, val, end)) = super::der_tlv::der_tlv(value, 0) else {
        return false;
    };
    // First content byte counts the unused trailing bits; the flags follow.
    if tag != 0x03 || end <= val + 1 {
        return false;
    }
    value[val + 1] & KEY_CERT_SIGN != 0
}
