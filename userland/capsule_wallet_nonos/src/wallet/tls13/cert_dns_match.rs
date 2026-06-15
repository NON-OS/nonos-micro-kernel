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
    let mut pos = 0usize;
    while pos + 2 <= cert.len() {
        let len = cert[pos + 1] as usize;
        if len < 128 {
            if pos + 2 + len > cert.len() {
                return false;
            }
            if cert[pos] == 0x82 && host_match(&cert[pos + 2..pos + 2 + len], host) {
                return true;
            }
            pos += 2 + len;
        } else {
            pos += 1;
        }
    }
    false
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
    host[host.len() - suffix.len() - 1] == b'.' && !host[..host.len() - suffix.len() - 1].contains(&b'.')
}

fn eq_ascii(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.eq_ignore_ascii_case(y))
}
