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

/// Run `f` with user memory reachable, and close the window again afterwards.
///
/// The guard closes on unwind as well as on return. This kernel aborts rather
/// than unwinds, so that costs nothing today, but it means the window cannot
/// be left open by an early exit added later.
#[inline(always)]
pub fn with_user_access<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let _window = Window::open();
    f()
}

struct Window;

impl Window {
    #[inline(always)]
    fn open() -> Self {
        #[cfg(target_arch = "x86_64")]
        super::smap::allow();
        #[cfg(target_arch = "aarch64")]
        super::pan::allow();
        Window
    }
}

impl Drop for Window {
    #[inline(always)]
    fn drop(&mut self) {
        #[cfg(target_arch = "x86_64")]
        super::smap::deny();
        #[cfg(target_arch = "aarch64")]
        super::pan::deny();
    }
}
