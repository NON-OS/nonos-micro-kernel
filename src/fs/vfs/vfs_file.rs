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

extern crate alloc;

use super::error::{VfsError, VfsResult};
use super::path_validate::validate_path;
use super::types::{FileMetadata, FileType};
use super::vfs_core::VirtualFileSystem;
use alloc::vec::Vec;
use spin::Mutex;

static VFS_OP_LOCK: Mutex<()> = Mutex::new(());

impl VirtualFileSystem {
    pub fn copy(&self, src: &str, dst: &str) -> VfsResult<()> {
        validate_path(src)?;
        validate_path(dst)?;
        let _lock = VFS_OP_LOCK.lock();
        let data: Vec<u8> = crate::fs::ramfs::NONOS_FILESYSTEM.read_file(src)?;
        crate::fs::ramfs::NONOS_FILESYSTEM.create_file(dst, &data)?;
        self.inner.lock().vfs_stats.copy_ops += 1;
        Ok(())
    }

    pub fn rename(&self, old_path: &str, new_path: &str) -> VfsResult<()> {
        validate_path(old_path)?;
        validate_path(new_path)?;
        let _lock = VFS_OP_LOCK.lock();
        crate::fs::ramfs::NONOS_FILESYSTEM.atomic_rename(old_path, new_path)?;
        self.inner.lock().vfs_stats.rename_ops += 1;
        Ok(())
    }

    pub fn unlink(&self, path: &str) -> VfsResult<()> {
        validate_path(path)?;
        if let Some(rel) = super::data_rel::data_rel(path) {
            crate::fs::blockfs_volume::remove(rel)
                .map_err(super::map_volume_err::map_volume_err)?;
            self.inner.lock().vfs_stats.unlink_ops += 1;
            return Ok(());
        }
        crate::fs::ramfs::NONOS_FILESYSTEM.delete_file(path)?;
        self.inner.lock().vfs_stats.unlink_ops += 1;
        Ok(())
    }

    pub fn write_file(&self, path: &str, data: &[u8]) -> VfsResult<()> {
        validate_path(path)?;
        let _lock = VFS_OP_LOCK.lock();
        if let Some(rel) = super::data_rel::data_rel(path) {
            return crate::fs::blockfs_volume::write_or_create(rel, data)
                .map_err(super::map_volume_err::map_volume_err);
        }
        self.ensure_parent_dirs(path)?;
        crate::fs::ramfs::NONOS_FILESYSTEM.write_or_create(path, data)?;
        Ok(())
    }

    pub fn stat(&self, path: &str) -> VfsResult<FileMetadata> {
        validate_path(path)?;
        let _lock = VFS_OP_LOCK.lock();
        if let Some(rel) = super::data_rel::data_rel(path) {
            return super::data_stat::data_stat(path, rel);
        }
        let is_dir = path.ends_with('/') || path.ends_with("/.dir");
        let size = if is_dir {
            0
        } else {
            match crate::fs::ramfs::NONOS_FILESYSTEM.read_file(path) {
                Ok(data) => data.len() as u64,
                Err(_) => return Err(VfsError::NotFound),
            }
        };
        let mut inode = 1u64;
        for byte in path.bytes() {
            inode = inode.wrapping_mul(31).wrapping_add(byte as u64);
        }
        let mode = if is_dir { 0o040755 } else { 0o100644 };
        let now = crate::time::timestamp_secs();
        Ok(FileMetadata {
            size,
            atime: now,
            mtime: now,
            ctime: now,
            file_type: if is_dir { FileType::Directory } else { FileType::File },
            mode,
            inode,
        })
    }
}
