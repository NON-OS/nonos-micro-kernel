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

use core::fmt;

pub struct PoisonError<T> {
    guard: T,
}

impl<T> PoisonError<T> {
    pub fn new(guard: T) -> Self {
        Self { guard }
    }

    pub fn into_inner(self) -> T {
        self.guard
    }

    pub fn get_ref(&self) -> &T {
        &self.guard
    }
}

impl<T> fmt::Debug for PoisonError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PoisonError")
    }
}

impl<T> fmt::Display for PoisonError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("poisoned lock")
    }
}

pub type LockResult<G> = Result<G, PoisonError<G>>;

pub enum TryLockError<T> {
    Poisoned(PoisonError<T>),
    WouldBlock,
}

pub type TryLockResult<G> = Result<G, TryLockError<G>>;
