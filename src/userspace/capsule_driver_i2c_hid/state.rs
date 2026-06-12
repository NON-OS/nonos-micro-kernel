// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use crate::services::lifecycle::CapsuleState;

static STATE: CapsuleState = CapsuleState::new();

pub(super) fn set_alive(pid: u32) {
    STATE.set_alive(pid);
}

pub fn shared_state() -> &'static CapsuleState {
    &STATE
}
