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

#[cfg(feature = "nonos-capsule-compositor")]
pub(super) fn spawn_compositor() {
    use crate::userspace::capsule_compositor as c;
    if c::shared_state().is_alive() {
        return;
    }
    super::super::boot::capsule(
        "COMPOSITOR",
        "compositor",
        c::spawn_compositor_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-compositor"))]
pub(super) fn spawn_compositor() {}
