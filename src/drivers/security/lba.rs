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

use super::error::DriverError;

pub fn validate_lba_range(lba: u64, count: u64, max_lba: u64) -> Result<(), DriverError> {
    if count == 0 {
        return Err(DriverError::LbaOutOfRange);
    }

    let end_lba = lba.checked_add(count).ok_or(DriverError::LbaOutOfRange)?;

    if end_lba > max_lba {
        return Err(DriverError::LbaOutOfRange);
    }

    Ok(())
}

pub fn validate_lba_range_with_size(
    lba: u64,
    count: u64,
    max_lba: u64,
    block_size: u32,
    max_transfer_bytes: usize,
) -> Result<usize, DriverError> {
    validate_lba_range(lba, count, max_lba)?;

    let total_bytes = count.checked_mul(block_size as u64).ok_or(DriverError::LbaOutOfRange)?;

    if total_bytes > max_transfer_bytes as u64 {
        return Err(DriverError::LbaOutOfRange);
    }

    Ok(total_bytes as usize)
}

pub fn is_lba_in_partition(lba: u64, partition_start: u64, partition_size: u64) -> bool {
    if partition_size == 0 {
        return false;
    }

    let partition_end = partition_start.saturating_add(partition_size);
    lba >= partition_start && lba < partition_end
}

pub fn validate_lba_in_partition(
    lba: u64,
    count: u64,
    partition_start: u64,
    partition_size: u64,
) -> Result<(), DriverError> {
    if count == 0 {
        return Err(DriverError::LbaOutOfRange);
    }

    if lba < partition_start {
        return Err(DriverError::LbaOutOfRange);
    }

    let relative_lba = lba.checked_sub(partition_start).ok_or(DriverError::LbaOutOfRange)?;

    let end_relative = relative_lba.checked_add(count).ok_or(DriverError::LbaOutOfRange)?;

    if end_relative > partition_size {
        return Err(DriverError::LbaOutOfRange);
    }

    Ok(())
}
