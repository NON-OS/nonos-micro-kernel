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

use spin::RwLock;

use super::error::{StorageError, StorageResult};
use super::stats;
use super::types::*;

static GLOBAL_QUOTA: RwLock<StorageQuota> = RwLock::new(StorageQuota {
    soft_limit: DEFAULT_MAX_STORAGE * 90 / 100,
    hard_limit: DEFAULT_MAX_STORAGE,
    current_usage: 0,
    file_limit: DEFAULT_MAX_FILES,
    file_count: 0,
    grace_period_secs: 7 * 24 * 60 * 60,
    exceeded_at: None,
});

pub fn get_quota() -> StorageQuota {
    let quota = GLOBAL_QUOTA.read();
    StorageQuota {
        soft_limit: quota.soft_limit,
        hard_limit: quota.hard_limit,
        current_usage: stats::get_total_storage_used(),
        file_limit: quota.file_limit,
        file_count: stats::get_inode_statistics().used_inodes,
        grace_period_secs: quota.grace_period_secs,
        exceeded_at: quota.exceeded_at,
    }
}

pub fn set_soft_limit(limit: usize) -> StorageResult<()> {
    if limit == 0 || limit > DEFAULT_MAX_STORAGE {
        return Err(StorageError::InvalidSize);
    }

    let mut quota = GLOBAL_QUOTA.write();
    if limit > quota.hard_limit {
        return Err(StorageError::InvalidSize);
    }

    quota.soft_limit = limit;
    Ok(())
}

pub fn set_hard_limit(limit: usize) -> StorageResult<()> {
    if limit == 0 || limit > DEFAULT_MAX_STORAGE {
        return Err(StorageError::InvalidSize);
    }

    let mut quota = GLOBAL_QUOTA.write();
    quota.hard_limit = limit;

    if quota.soft_limit > limit {
        quota.soft_limit = limit * 90 / 100;
    }

    Ok(())
}

pub fn set_file_limit(limit: usize) -> StorageResult<()> {
    if limit == 0 || limit > DEFAULT_MAX_FILES {
        return Err(StorageError::InvalidSize);
    }

    let mut quota = GLOBAL_QUOTA.write();
    quota.file_limit = limit;
    Ok(())
}

pub fn set_grace_period(seconds: u64) {
    let mut quota = GLOBAL_QUOTA.write();
    quota.grace_period_secs = seconds;
}

pub fn check_can_allocate(bytes: usize) -> StorageResult<()> {
    let current_usage = stats::get_total_storage_used();
    let quota = GLOBAL_QUOTA.read();

    if current_usage + bytes > quota.hard_limit {
        return Err(StorageError::QuotaExceeded);
    }

    Ok(())
}

pub fn check_can_create_file() -> StorageResult<()> {
    let inode_stats = stats::get_inode_statistics();
    let quota = GLOBAL_QUOTA.read();

    if inode_stats.used_inodes >= quota.file_limit {
        return Err(StorageError::InodeExhausted);
    }

    Ok(())
}

pub fn is_soft_limit_exceeded() -> bool {
    let current_usage = stats::get_total_storage_used();
    let quota = GLOBAL_QUOTA.read();
    current_usage >= quota.soft_limit
}

pub fn is_hard_limit_exceeded() -> bool {
    let current_usage = stats::get_total_storage_used();
    let quota = GLOBAL_QUOTA.read();
    current_usage >= quota.hard_limit
}

pub fn update_exceeded_timestamp(timestamp: u64) {
    let mut quota = GLOBAL_QUOTA.write();
    if quota.exceeded_at.is_none() && is_soft_limit_exceeded() {
        quota.exceeded_at = Some(timestamp);
    } else if !is_soft_limit_exceeded() {
        quota.exceeded_at = None;
    }
}

pub fn is_grace_period_expired(current_time: u64) -> bool {
    let quota = GLOBAL_QUOTA.read();
    if let Some(exceeded_at) = quota.exceeded_at {
        current_time.saturating_sub(exceeded_at) >= quota.grace_period_secs
    } else {
        false
    }
}

pub fn reset_quota() {
    let mut quota = GLOBAL_QUOTA.write();
    *quota = StorageQuota::default();
}

pub fn get_remaining_capacity() -> (usize, usize) {
    let current_usage = stats::get_total_storage_used();
    let inode_stats = stats::get_inode_statistics();
    let quota = GLOBAL_QUOTA.read();

    let remaining_bytes = quota.hard_limit.saturating_sub(current_usage);
    let remaining_files = quota.file_limit.saturating_sub(inode_stats.used_inodes);

    (remaining_bytes, remaining_files)
}
