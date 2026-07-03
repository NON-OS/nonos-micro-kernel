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

use super::path_tok::Tok;

impl Tok<'_> {
    // One signed decimal with optional exponent, scanned then parsed.
    pub(super) fn num(&mut self) -> Option<f32> {
        self.skip_sep();
        let b = self.s.as_bytes();
        let start = self.i;
        let mut j = self.i;
        if j < b.len() && (b[j] == b'+' || b[j] == b'-') {
            j += 1;
        }
        let mut seen_dot = false;
        while j < b.len() && (b[j].is_ascii_digit() || (b[j] == b'.' && !seen_dot)) {
            seen_dot |= b[j] == b'.';
            j += 1;
        }
        if j < b.len() && (b[j] == b'e' || b[j] == b'E') {
            let mut k = j + 1;
            if k < b.len() && (b[k] == b'+' || b[k] == b'-') {
                k += 1;
            }
            while k < b.len() && b[k].is_ascii_digit() {
                k += 1;
            }
            j = k;
        }
        let v = self.s.get(start..j)?.parse::<f32>().ok()?;
        self.i = j;
        if v.is_finite() {
            Some(v)
        } else {
            None
        }
    }
}
