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
//! Checking a pack arrived intact.

use crate::sha1::Sha1;

use super::error::PackError;

/// Verify the twenty byte SHA-1 a pack ends with.
///
/// It covers every byte before it. Without this a pack that was damaged or
/// altered in transit is read anyway, and the damage only surfaces later as
/// an object whose id does not match, or worse, as one whose id does.
pub(super) fn verify(pack: &[u8]) -> Result<(), PackError> {
    let body = pack.len().checked_sub(20).ok_or(PackError::Truncated)?;
    let (data, trailer) = pack.split_at(body);
    if Sha1::digest(data) != trailer {
        return Err(PackError::Checksum);
    }
    Ok(())
}
