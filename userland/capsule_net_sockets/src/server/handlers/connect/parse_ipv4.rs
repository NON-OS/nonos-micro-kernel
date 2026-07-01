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

pub fn parse_ipv4(host: &[u8]) -> Option<[u8; 4]> {
    let s = core::str::from_utf8(host).ok()?;
    let mut out = [0u8; 4];
    let mut count = 0usize;
    for part in s.split('.') {
        if count == 4 {
            return None;
        }
        out[count] = part.parse::<u8>().ok()?;
        count += 1;
    }
    (count == 4).then_some(out)
}
