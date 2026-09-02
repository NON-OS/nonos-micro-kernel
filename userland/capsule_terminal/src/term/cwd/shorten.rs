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

use super::strip_home::strip_home;

/// The display form of a path: `~` in place of the home prefix, or the path
/// unchanged when it lies outside. Callers share this so the prompt, the tab
/// label and the block header cannot name one directory three ways.
pub fn shorten<'a>(cwd: &'a [u8], home: &[u8], out: &'a mut [u8]) -> &'a [u8] {
    match strip_home(cwd, home) {
        Some(tail) => {
            let n = tail.len().min(out.len().saturating_sub(1));
            out[0] = b'~';
            out[1..1 + n].copy_from_slice(&tail[..n]);
            &out[..1 + n]
        }
        None => cwd,
    }
}
