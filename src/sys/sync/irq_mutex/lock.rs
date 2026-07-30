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

use crate::arch::cpu::{disable_interrupts, interrupts_enabled};
use core::mem::ManuallyDrop;

use super::guard::IrqMutexGuard;
use super::state::IrqMutex;

impl<T> IrqMutex<T> {
    pub fn lock(&self) -> IrqMutexGuard<'_, T> {
        let were_enabled = interrupts_enabled();
        if were_enabled {
            disable_interrupts();
        }
        let guard = self.inner.lock();
        IrqMutexGuard { inner: ManuallyDrop::new(guard), restore: were_enabled }
    }
}
