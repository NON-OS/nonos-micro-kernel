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

use crate::arch::x86_64::idt::enable;

use super::guard::IrqMutexGuard;

impl<'a, T> Drop for IrqMutexGuard<'a, T> {
    fn drop(&mut self) {
        // Release the inner spin lock first so a peer CPU can grab it,
        // then restore the prior local IF state if it was on at lock().
        self.inner.take();
        if self.restore {
            enable();
        }
    }
}
