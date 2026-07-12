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

use super::super::types::{VtdError, PAGE_SIZE_4K};

pub(crate) fn validate_range(iova: u64, size: usize) -> Result<usize, VtdError> {
    let granule = PAGE_SIZE_4K as u64;
    if size == 0 {
        return Err(VtdError::SizeMisaligned);
    }
    if iova & (granule - 1) != 0 {
        return Err(VtdError::AddressMisaligned);
    }
    if (size as u64) & (granule - 1) != 0 {
        return Err(VtdError::SizeMisaligned);
    }
    let end = iova.checked_add(size as u64).ok_or(VtdError::RangeOutOfBounds)?;
    let _ = end;
    Ok(size / PAGE_SIZE_4K)
}
