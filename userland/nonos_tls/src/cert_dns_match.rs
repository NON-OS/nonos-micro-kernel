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

pub fn matches(cert: &[u8], host: &[u8]) -> bool {
    let Some(san) = san_value(cert) else {
        return false;
    };
    let mut pos = 0usize;
    while pos + 2 <= san.len() {
        let len = san[pos + 1] as usize;
        if len < 128 {
            if pos + 2 + len > san.len() {
                return false;
            }
            if san[pos] == 0x82 && host_match(&san[pos + 2..pos + 2 + len], host) {
                return true;
            }
            pos += 2 + len;
        } else {
            pos += 1;
        }
    }
    false
}

fn san_value(cert: &[u8]) -> Option<&[u8]> {
    let needle = [0x06u8, 0x03, 0x55, 0x1D, 0x11];
    let p = cert.windows(5).position(|w| w == needle)?;
    let (tag, v, e) = super::der_tlv::der_tlv(cert, p + 5)?;
    let (octag, octv, _) =
        if tag == 0x01 { super::der_tlv::der_tlv(cert, e)? } else { (tag, v, e) };
    if octag != 0x04 {
        return None;
    }
    let (seqtag, sv, se) = super::der_tlv::der_tlv(cert, octv)?;
    if seqtag != 0x30 {
        return None;
    }
    Some(&cert[sv..se])
}

fn host_match(name: &[u8], host: &[u8]) -> bool {
    if eq_ascii(name, host) {
        return true;
    }
    name.len() > 2 && name[0] == b'*' && name[1] == b'.' && wildcard(&name[2..], host)
}

fn wildcard(suffix: &[u8], host: &[u8]) -> bool {
    if host.len() <= suffix.len() || !eq_ascii(&host[host.len() - suffix.len()..], suffix) {
        return false;
    }
    host[host.len() - suffix.len() - 1] == b'.'
        && !host[..host.len() - suffix.len() - 1].contains(&b'.')
}

fn eq_ascii(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.eq_ignore_ascii_case(y))
}
