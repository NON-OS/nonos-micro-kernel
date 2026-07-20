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

// Advisory file locks. The VFS models none, so every lock succeeds as a
// no-op: a single-writer capsule sees no contention, and try_lock never
// reports the file as already held.

use super::File;
use crate::fs::TryLockError;
use crate::io;

impl File {
    pub fn lock(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn lock_shared(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn try_lock(&self) -> Result<(), TryLockError> {
        Ok(())
    }

    pub fn try_lock_shared(&self) -> Result<(), TryLockError> {
        Ok(())
    }

    pub fn unlock(&self) -> io::Result<()> {
        Ok(())
    }
}
