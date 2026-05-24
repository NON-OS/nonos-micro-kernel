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

//! Shared liveness state for the virtio-rng driver capsule. Same
//! shape as every other userland service capsule so the supervisor
//! and the boot harness can probe it without per-capsule wiring.

use crate::services::lifecycle::CapsuleState;

static STATE: CapsuleState = CapsuleState::new();

pub(super) fn set_alive(pid: u32) {
    STATE.set_alive(pid);
}

pub fn shared_state() -> &'static CapsuleState {
    &STATE
}
