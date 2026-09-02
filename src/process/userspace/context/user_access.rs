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

//! Kept under the name the usercopy paths already call. The window itself,
//! and the guard that closes it on every path out, belong to the arch
//! boundary, which is where both architectures state what they mean by it.

#[inline(always)]
pub fn with_user_access<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    crate::arch::user_access::with_user_access(f)
}
