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

use super::types::{OpenFd, Store, StoreError, StoreResult};

impl Store {
    pub(super) fn find(&self, path: &str) -> Option<usize> {
        self.files.iter().position(|f| f.name == path)
    }

    pub(super) fn entry(&self, fd: u32, owner_pid: u32) -> StoreResult<&OpenFd> {
        let idx = fd as usize;
        if idx >= self.fds.len() {
            return Err(StoreError::BadFd);
        }
        match self.fds[idx].as_ref() {
            Some(e) if e.owner_pid == owner_pid => Ok(e),
            Some(_) => Err(StoreError::BadFd),
            None => Err(StoreError::BadFd),
        }
    }

    pub(super) fn slot_mut(&mut self, fd: u32, owner_pid: u32) -> StoreResult<&mut Option<OpenFd>> {
        let idx = fd as usize;
        if idx >= self.fds.len() {
            return Err(StoreError::BadFd);
        }
        match self.fds[idx].as_ref() {
            Some(e) if e.owner_pid == owner_pid => Ok(&mut self.fds[idx]),
            Some(_) => Err(StoreError::BadFd),
            None => Err(StoreError::BadFd),
        }
    }
}
