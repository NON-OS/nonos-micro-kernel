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

// Yields the path and whether consent was given, accepting --yes on either
// side of the path. An unrecognised flag is rejected instead of being taken
// for a path, which would otherwise fail far away as a bad package.
pub(super) fn install<'a>(rest: &[&'a [u8]]) -> Option<(&'a [u8], bool)> {
    let mut path: Option<&'a [u8]> = None;
    let mut yes = false;
    for arg in rest {
        if *arg == b"--yes" {
            yes = true;
        } else if arg.starts_with(b"-") {
            return None;
        } else if path.is_none() {
            path = Some(arg);
        }
    }
    Some((path?, yes))
}
