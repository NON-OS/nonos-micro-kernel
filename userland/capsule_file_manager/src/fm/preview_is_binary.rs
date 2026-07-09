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

pub fn is_binary(bytes: &[u8]) -> bool {
    let sample = &bytes[..bytes.len().min(1024)];
    if sample.is_empty() {
        return false;
    }
    let mut suspicious = 0usize;
    for &b in sample {
        if b == 0 {
            return true;
        }
        let printable = (0x20..=0x7e).contains(&b) || matches!(b, b'\n' | b'\r' | b'\t');
        if !printable {
            suspicious += 1;
        }
    }
    suspicious * 100 / sample.len() > 30
}
