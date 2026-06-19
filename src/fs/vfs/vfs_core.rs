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

use super::error::VfsResult;
use super::open_file::secure_zeroize_string;
use super::path_validate::validate_path;
use super::types::{FileSystemType, IoStatistics, MountPoint, VfsStatistics, MAX_MOUNTS};
use alloc::{string::String, vec::Vec};
use core::sync::atomic::{compiler_fence, Ordering};

#[derive(Debug)]
pub(super) struct VirtualFileSystemInner {
    pub mounts: Vec<MountPoint>,
    pub pending_ops: usize,
    pub io_stats: IoStatistics,
    pub vfs_stats: VfsStatistics,
}

#[derive(Debug)]
pub struct VirtualFileSystem {
    pub(super) inner: spin::Mutex<VirtualFileSystemInner>,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        Self {
            inner: spin::Mutex::new(VirtualFileSystemInner {
                mounts: Vec::new(),
                pending_ops: 0,
                io_stats: IoStatistics::default(),
                vfs_stats: VfsStatistics::default(),
            }),
        }
    }

    pub fn sync_metadata(&self) {
        let g = self.inner.lock();
        for mount in &g.mounts {
            let _ = crate::fs::ramfs::NONOS_FILESYSTEM.sync(&mount.mount_path);
        }
        compiler_fence(Ordering::SeqCst);
    }

    pub fn process_pending_operations(&self, max_ops: usize) -> usize {
        let mut g = self.inner.lock();
        let to_process = core::cmp::min(g.pending_ops, max_ops);
        g.pending_ops = g.pending_ops.saturating_sub(to_process);
        to_process
    }

    pub fn mount(&self, mount_path: &str, fs_type: FileSystemType) {
        if validate_path(mount_path).is_err() {
            return;
        }
        let mut g = self.inner.lock();
        if g.mounts.len() < MAX_MOUNTS {
            g.mounts.push(MountPoint { mount_path: String::from(mount_path), filesystem: fs_type });
            g.vfs_stats.mounts += 1;
        }
    }

    pub fn mounts(&self) -> Vec<MountPoint> {
        self.inner.lock().mounts.clone()
    }

    pub fn exists(&self, path: &str) -> bool {
        validate_path(path).is_ok() && crate::fs::ramfs::NONOS_FILESYSTEM.exists(path)
    }

    pub fn read_file(&self, path: &str) -> VfsResult<Vec<u8>> {
        validate_path(path)?;
        if let Some(rel) = super::data_rel::data_rel(path) {
            return crate::fs::blockfs_volume::read_all(rel)
                .map_err(super::map_volume_err::map_volume_err);
        }
        crate::fs::ramfs::NONOS_FILESYSTEM.read_file(path).map_err(super::error::VfsError::from)
    }

    pub fn stats(&self) -> VfsStatistics {
        self.inner.lock().vfs_stats.clone()
    }

    pub fn create_directory(&self, path: &str) -> VfsResult<()> {
        validate_path(path)?;
        crate::fs::ramfs::create_dir(path).map_err(super::error::VfsError::from)
    }

    pub fn clear_all(&self) {
        let mut inner = self.inner.lock();
        for mount in inner.mounts.iter_mut() {
            secure_zeroize_string(&mut mount.mount_path);
        }
        inner.mounts.clear();
        inner.pending_ops = 0;
        inner.io_stats = IoStatistics::default();
        inner.vfs_stats = VfsStatistics::default();
        compiler_fence(Ordering::SeqCst);
    }
}
