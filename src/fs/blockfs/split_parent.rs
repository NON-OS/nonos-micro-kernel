// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::BlockFsError;

pub(super) fn split_parent(path: &[u8]) -> Result<(&[u8], &[u8]), BlockFsError> {
    let end = path.iter().rposition(|&b| b != b'/').ok_or(BlockFsError::InvalidName)?;
    let trimmed = &path[..=end];
    match trimmed.iter().rposition(|&b| b == b'/') {
        Some(slash) => {
            let name = &trimmed[slash + 1..];
            if name.is_empty() {
                return Err(BlockFsError::InvalidName);
            }
            Ok((&trimmed[..slash], name))
        }
        None => Ok((b"", trimmed)),
    }
}
