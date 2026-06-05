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

use core::ops::{Deref, DerefMut};

use super::guard::{IrqRwLockReadGuard, IrqRwLockWriteGuard};

impl<'a, T> Deref for IrqRwLockReadGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &**self.inner
    }
}

impl<'a, T> Deref for IrqRwLockWriteGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &**self.inner
    }
}

impl<'a, T> DerefMut for IrqRwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut **self.inner
    }
}
