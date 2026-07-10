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

//! Guard the paths we are willing to hand to the filesystem service.

use super::constants::MAX_NAME;

/// A path is well formed when it is non-empty, fits the wire length limit,
/// starts at the root, and holds no interior NUL. Anything else is refused
/// before it ever reaches the server.
pub(super) fn is_valid(path: &[u8]) -> bool {
    !path.is_empty() && path.len() <= MAX_NAME && path[0] == b'/' && !path.contains(&0)
}
