// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! #403 service registration (caller_can_register in
//! src/services/registry/auth/caller_can_register.rs). The old rule let any
//! caller with pid <= 64 register without RegisterService; the fix removes that
//! shortcut. `None` is kernel context, `Some(pid)` a capsule.

/// The old rule: kernel context, a low pid, or the right, is enough.
pub fn can_register_old(caller: Option<u32>, has_right: bool) -> bool {
    match caller {
        None => true,
        Some(pid) => pid <= 64 || has_right,
    }
}

/// The fixed rule: kernel context, or the right, and nothing else.
pub fn can_register_new(caller: Option<u32>, has_right: bool) -> bool {
    match caller {
        None => true,
        Some(_) => has_right,
    }
}
