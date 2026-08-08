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

//! Releasing port grants where there are no ports.
//!
//! Only x86_64 has a separate I/O address space, so the grant table above is
//! built only there and the syscall layer answers ENOSYS everywhere else.
//! That leaves the two release paths, which run from process teardown and
//! device removal and are not asking for a port: they are asking that this
//! pid or device hold none. On an architecture that never issued one the
//! answer is already yes.
//!
//! Written out rather than left to a `cfg` at each call site so teardown
//! stays one path on every target. A caller that forgets the gate is a leak
//! on x86_64 and a build error on aarch64, which is the wrong way round.

/// Drop every port grant held by `pid`, and report how many went. None can,
/// so none did.
pub fn pio_release_all_for_pid(pid: u32) -> usize {
    let _ = pid;
    0
}

/// Drop every port grant `pid` holds over `device_id`, and report how many
/// went. None can, so none did.
pub fn pio_release_for_device(pid: u32, device_id: u64) -> usize {
    let _ = (pid, device_id);
    0
}
