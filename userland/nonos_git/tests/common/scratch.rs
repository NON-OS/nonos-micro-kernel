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

//! A scratch directory that removes itself when the test ends.

use std::fs;
use std::path::PathBuf;

use super::storage::DirStorage;

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
