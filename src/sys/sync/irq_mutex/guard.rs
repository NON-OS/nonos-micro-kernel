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

use spin::MutexGuard;

// Drop semantics live in `drop.rs`: release the inner spin lock first
// (so peer CPUs can grab it) then restore the prior local IF state if
// it was on when lock() ran.
pub struct IrqMutexGuard<'a, T> {
    pub(super) inner: Option<MutexGuard<'a, T>>,
    pub(super) restore: bool,
}
