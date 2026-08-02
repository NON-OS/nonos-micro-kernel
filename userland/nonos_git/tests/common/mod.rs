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

//! A `Storage` over a real directory, shared by the repository tests.
//!
//! The crate itself is `no_std`; this lives on the test side so the tests
//! exercise the same public API a capsule uses, over an ordinary filesystem
//! that real `git` can also be pointed at.

use std::fs;
use std::path::PathBuf;

use nonos_git::{Storage, StorageError};

pub struct DirStorage {
    root: PathBuf,
}

impl DirStorage {
    pub fn new(root: PathBuf) -> DirStorage {
        DirStorage { root }
    }
}

impl Storage for DirStorage {
    fn read(&self, path: &str) -> Result<Vec<u8>, StorageError> {
        fs::read(self.root.join(path)).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => StorageError::NotFound,
            _ => StorageError::Io,
        })
    }

    fn write(&mut self, path: &str, data: &[u8]) -> Result<(), StorageError> {
        let full = self.root.join(path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).map_err(|_| StorageError::Io)?;
        }
        fs::write(full, data).map_err(|_| StorageError::Io)
    }

    fn exists(&self, path: &str) -> bool {
        self.root.join(path).exists()
    }

    fn create_dir_all(&mut self, path: &str) -> Result<(), StorageError> {
        fs::create_dir_all(self.root.join(path)).map_err(|_| StorageError::Io)
    }

    fn read_dir(&self, path: &str) -> Result<Vec<String>, StorageError> {
        let mut out = Vec::new();
        for entry in fs::read_dir(self.root.join(path)).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => StorageError::NotFound,
            _ => StorageError::Io,
        })? {
            out.push(
                entry.map_err(|_| StorageError::Io)?.file_name().to_string_lossy().into_owned(),
            );
        }
        Ok(out)
    }

    fn is_dir(&self, path: &str) -> bool {
        self.root.join(path).is_dir()
    }
}

/// A scratch directory that removes itself when the test ends.
pub struct Scratch {
    pub path: PathBuf,
}

impl Scratch {
    pub fn new(tag: &str) -> Scratch {
        let mut path = std::env::temp_dir();
        // The pid and tag keep concurrent tests apart without needing a clock.
        path.push(format!("nonos_git_{}_{}", std::process::id(), tag));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("scratch dir");
        Scratch { path }
    }

    pub fn storage(&self) -> DirStorage {
        DirStorage::new(self.path.clone())
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
