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

pub mod api;
pub mod blockfs;
pub mod blockfs_volume;
pub mod cache;
pub mod cryptoblock;
pub mod cryptofs;
// `devfs`, `ext4`, and `sysfs` are block-backed/host-driver-backed
// surfaces from the legacy tree. The microkernel filesystem path is
// `ramfs` (in-kernel) plus `ramfs_capsule` (IPC-routed). devfs needs
// `crate::tty` and `crate::drivers::{block,usb,keyboard,keyboard_buffer}`,
// ext4 needs `crate::drivers::block`, sysfs scans the device-driver
// inventory; all three target legacy-only.
pub mod fd;
pub mod path;
pub mod pipe;
pub mod procfs;
pub mod ramfs;
pub mod ramfs_capsule;
pub mod storage;
pub mod utils;
pub mod vfs;
pub mod vfs_capsule;

mod errors;
mod internal;
mod manager;
mod mapping;
mod ops;

pub use cryptofs as nonos_crypto;
pub use ramfs as nonos_filesystem;
pub use vfs as nonos_vfs;
pub use vfs as nvfs;

pub use vfs::{
    get_vfs, get_vfs_mut, init_vfs, CowPageRef, DeviceOperations, FileBuffer, FileCacheEntry,
    FileMetadata, FileMode, FileSystemOperations, FileSystemType, FileType, IoOperation, IoRequest,
    IoStatistics, MountPoint, VfsError, VfsInode, VfsResult, VirtualFileSystem,
};

pub use cryptofs::{
    clear_crypto_state, create_encrypted_file, create_ephemeral_file, delete_encrypted,
    get_cryptofs, init_cryptofs, nonce_counter_warning, read_encrypted, rotate_file_key,
    write_encrypted, CryptoFileSystem, CryptoFsError, CryptoFsStatistics, CryptoResult,
};

pub use ramfs::{FsError, FsResult};

pub use fd::{
    close_file_descriptor, fstat_file_syscall, open_file_syscall, read_file_descriptor,
    rmdir_syscall, stat_file_syscall, sync_all, unlink_syscall, write_file_descriptor, FdError,
    FdResult,
};

pub use path::{
    components, cstr_to_string, extension, file_name, is_absolute, is_relative, join,
    join_normalize, join_secure, normalize_path, parent, validate_path, validate_path_secure,
    PathError, PathResult, MAX_PATH_LEN,
};

pub use cache::{
    clear_all_caches, get_cache_hit_ratio, get_cache_statistics, get_full_cache_statistics,
    init_all_caches, CacheStats, CACHE_STATS,
};

pub use utils::{
    classify_file, is_hidden_file, is_sensitive_file, list_hidden_files, scan_files_with_config,
    scan_for_sensitive_files, FileCategory, FileClassification, ScanConfig, ScanResult,
    SensitivityLevel, UtilsError, UtilsResult,
};

pub use storage::{
    get_filesystem_breakdown, get_inode_stats, get_storage_health, get_storage_stats,
    get_storage_usage_percent, get_total_available_bytes, get_total_used_bytes,
    FilesystemBreakdown, InodeStats, StorageError, StorageHealth, StorageHealthStatus,
    StorageQuota, StorageResult, StorageStats,
};

pub use errors::{FsSubsystemError, FsSubsystemResult};
pub use manager::{
    get_filesystem_manager, init_filesystem_manager, FileSystemManager, FileSystemManagerStats,
};
pub use mapping::{FileMapping, MappingProtection};
pub use ops::{
    chmod, chown, clear_caches, init, is_directory, link, mkdir, mknod, mount,
    process_pending_operations, read_file, read_file_bytes, readlink, rename, rmdir,
    run_filesystem_sync, set_times, set_times_at, symlink, truncate, umount, unlink, write_file,
};

pub use api::{
    allocate_fd, get_file_size, get_pipe_buffer_size, get_process_fd, get_process_fds, is_pipe_fd,
    pread, pwrite, register_pipe_reader, register_pipe_writer, set_cloexec, set_pipe_buffer_size,
    unregister_pipe_fd, FdInfo,
};
