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

use core::sync::atomic::{AtomicU64, Ordering};

use super::types::*;
use crate::fs::cache;
use crate::fs::cryptofs;
use crate::fs::ramfs::NONOS_FILESYSTEM;

static ALLOCATION_FAILURES: AtomicU64 = AtomicU64::new(0);
static IO_ERRORS: AtomicU64 = AtomicU64::new(0);

pub fn calculate_storage_stats() -> StorageStats {
    let ramfs_used = NONOS_FILESYSTEM.storage_used();
    let ramfs_files = NONOS_FILESYSTEM.file_count();

    let cryptofs_used = cryptofs::get_cryptofs().map(|fs| fs.storage_used()).unwrap_or(0);

    let cryptofs_files = cryptofs::get_cryptofs().map(|fs| fs.list_files().len()).unwrap_or(0);

    let cache_stats = cache::get_full_cache_statistics();
    let cache_bytes = cache_stats.bytes_cached;

    let used_bytes = ramfs_used + cryptofs_used + cache_bytes;
    let total_bytes = DEFAULT_MAX_STORAGE;
    let available_bytes = total_bytes.saturating_sub(used_bytes);

    let total_blocks = total_bytes / BLOCK_SIZE;
    let used_blocks = (used_bytes + BLOCK_SIZE - 1) / BLOCK_SIZE;
    let free_blocks = total_blocks.saturating_sub(used_blocks);

    let file_count = ramfs_files + cryptofs_files;
    let directory_count = count_directories();

    StorageStats {
        total_bytes,
        used_bytes,
        available_bytes,
        file_count,
        directory_count,
        block_size: BLOCK_SIZE,
        total_blocks,
        used_blocks,
        free_blocks,
    }
}

pub fn get_total_storage_used() -> usize {
    let ramfs_used = NONOS_FILESYSTEM.storage_used();
    let cryptofs_used = cryptofs::get_cryptofs().map(|fs| fs.storage_used()).unwrap_or(0);
    ramfs_used + cryptofs_used
}

pub fn get_total_storage_available() -> usize {
    DEFAULT_MAX_STORAGE.saturating_sub(get_total_storage_used())
}

pub fn get_usage_percentage() -> f32 {
    let used = get_total_storage_used();
    if DEFAULT_MAX_STORAGE == 0 {
        return 0.0;
    }
    (used as f32 / DEFAULT_MAX_STORAGE as f32) * 100.0
}

pub fn get_breakdown_by_filesystem() -> FilesystemBreakdown {
    let ramfs_bytes = NONOS_FILESYSTEM.storage_used();
    let ramfs_files = NONOS_FILESYSTEM.file_count();

    let cryptofs_bytes = cryptofs::get_cryptofs().map(|fs| fs.storage_used()).unwrap_or(0);

    let cryptofs_files = cryptofs::get_cryptofs().map(|fs| fs.list_files().len()).unwrap_or(0);

    let cache_stats = cache::get_full_cache_statistics();
    let cache_bytes = cache_stats.bytes_cached;

    let metadata_bytes = estimate_metadata_overhead(ramfs_files + cryptofs_files);

    FilesystemBreakdown {
        ramfs_bytes,
        ramfs_files,
        cryptofs_bytes,
        cryptofs_files,
        cache_bytes,
        metadata_bytes,
    }
}

pub fn check_storage_health() -> StorageHealth {
    let stats = calculate_storage_stats();
    let inode_stats = get_inode_statistics();

    let usage_percent = stats.usage_percent();
    let inode_usage_percent = inode_stats.usage_percent();
    let fragmentation_percent = estimate_fragmentation();

    let low_space = usage_percent >= WARNING_THRESHOLD_PERCENT;
    let low_inodes = inode_usage_percent >= WARNING_THRESHOLD_PERCENT;
    let high_fragmentation = fragmentation_percent >= 30.0;

    let allocation_failures = ALLOCATION_FAILURES.load(Ordering::Relaxed);
    let io_errors = IO_ERRORS.load(Ordering::Relaxed);

    let issues =
        StorageIssues { low_space, low_inodes, high_fragmentation, allocation_failures, io_errors };

    let status = determine_health_status(usage_percent, inode_usage_percent, &issues);

    StorageHealth { status, usage_percent, inode_usage_percent, fragmentation_percent, issues }
}

pub fn get_inode_statistics() -> InodeStats {
    let ramfs_files = NONOS_FILESYSTEM.file_count();
    let cryptofs_files = cryptofs::get_cryptofs().map(|fs| fs.list_files().len()).unwrap_or(0);

    let used_inodes = ramfs_files + cryptofs_files + count_directories();
    let free_inodes = DEFAULT_MAX_FILES.saturating_sub(used_inodes);

    InodeStats { total_inodes: DEFAULT_MAX_FILES, used_inodes, free_inodes, inode_size: INODE_SIZE }
}

pub fn record_allocation_failure() {
    ALLOCATION_FAILURES.fetch_add(1, Ordering::Relaxed);
}

pub fn record_io_error() {
    IO_ERRORS.fetch_add(1, Ordering::Relaxed);
}

pub fn reset_error_counters() {
    ALLOCATION_FAILURES.store(0, Ordering::Relaxed);
    IO_ERRORS.store(0, Ordering::Relaxed);
}

pub fn get_error_counts() -> (u64, u64) {
    (ALLOCATION_FAILURES.load(Ordering::Relaxed), IO_ERRORS.load(Ordering::Relaxed))
}

fn count_directories() -> usize {
    let files = NONOS_FILESYSTEM.list_files();
    let mut dir_count = 0;
    for file in &files {
        if file.ends_with("/.dir") || file.ends_with(".dir") {
            dir_count += 1;
        }
    }
    dir_count
}

fn estimate_metadata_overhead(file_count: usize) -> usize {
    file_count * INODE_SIZE
}

fn estimate_fragmentation() -> f32 {
    let stats = cache::get_full_cache_statistics();
    if stats.evictions == 0 {
        return 0.0;
    }
    let ratio = stats.evictions as f32 / (stats.hits + stats.misses + 1) as f32;
    (ratio * 100.0).min(100.0)
}

fn determine_health_status(
    usage_percent: f32,
    inode_usage_percent: f32,
    issues: &StorageIssues,
) -> StorageHealthStatus {
    if usage_percent >= CRITICAL_THRESHOLD_PERCENT
        || inode_usage_percent >= CRITICAL_THRESHOLD_PERCENT
    {
        return StorageHealthStatus::Critical;
    }

    if issues.io_errors > 10 || issues.allocation_failures > 10 {
        return StorageHealthStatus::Degraded;
    }

    if usage_percent >= WARNING_THRESHOLD_PERCENT
        || inode_usage_percent >= WARNING_THRESHOLD_PERCENT
        || issues.has_issues()
    {
        return StorageHealthStatus::Warning;
    }

    StorageHealthStatus::Healthy
}
