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

pub fn cert_valid_now(cert: &[u8], now: u64) -> bool {
    let mut pos = 0usize;
    while pos + 4 <= cert.len() {
        let tag = cert[pos];
        let len = cert[pos + 1] as usize;
        if len < 128 && pos + 2 + len <= cert.len() && (tag == 0x17 || tag == 0x18) {
            if pair_valid(cert, pos, now) {
                return true;
            }
        }
        pos += if len < 128 { 2 + len } else { 1 };
    }
    false
}

fn pair_valid(cert: &[u8], pos: usize, now: u64) -> bool {
    let len1 = cert[pos + 1] as usize;
    let end1 = pos + 2 + len1;
    if end1 + 2 > cert.len() {
        return false;
    }
    let len2 = cert[end1 + 1] as usize;
    if len2 >= 128 || end1 + 2 + len2 > cert.len() {
        return false;
    }
    let Some(from) = super::cert_time_value::cert_time_value(cert[pos], &cert[pos + 2..end1]) else { return false };
    let Some(until) = super::cert_time_value::cert_time_value(cert[end1], &cert[end1 + 2..end1 + 2 + len2]) else { return false };
    from <= now && now <= until
}
