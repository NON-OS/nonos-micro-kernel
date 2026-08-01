// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Retention of a dead capsule's `proc.<pid>` stdout inbox.
//!
//! A capsule's mirrored stdout lands in its own `proc.<pid>` inbox, but the
//! only consumer is the parent that launched it, which polls. `finalize`
//! removes the PCB and every inbox the instant the exit is drained, which is
//! within a scheduler tick of the exit, so a tool that runs for less than one
//! of the parent's poll intervals would lose all of its output and the parent
//! gate in `sys_proc_output` would fail for want of a PCB to read a parent
//! from. The mailbox therefore outlives the process: this map keeps the
//! parent pid readable and holds the inbox open until the parent drains it
//! dry. Capped and evict-oldest, like `REAP_LOG`, so children whose parent
//! never drains cannot grow it without bound.

use alloc::collections::BTreeMap;
use spin::Mutex;

use crate::process::core::Pid;

static RETAINED: Mutex<BTreeMap<Pid, Pid>> = Mutex::new(BTreeMap::new());
const RETAINED_CAP: usize = 64;

pub(crate) fn retain(pid: Pid, parent: Pid) {
    let evicted = {
        let mut map = RETAINED.lock();
        let evicted = if map.len() >= RETAINED_CAP {
            map.keys().next().copied()
        } else {
            None
        };
        if let Some(old) = evicted {
            map.remove(&old);
        }
        map.insert(pid, parent);
        evicted
    };
    if let Some(old) = evicted {
        drop_inbox(old);
    }
}

pub(crate) fn is_retained(pid: Pid) -> bool {
    RETAINED.lock().contains_key(&pid)
}

/// The parent a dead capsule was launched by, for as long as its stdout is
/// still retained. Lets the `sys_proc_output` gate outlive the PCB.
pub fn retained_parent(pid: Pid) -> Option<Pid> {
    RETAINED.lock().get(&pid).copied()
}

/// Drop a dead capsule's retained stdout inbox once its parent has drained it.
pub fn release(pid: Pid) {
    let retained = RETAINED.lock().remove(&pid).is_some();
    if retained {
        drop_inbox(pid);
    }
}

fn drop_inbox(pid: Pid) {
    let _ = crate::ipc::nonos_inbox::unregister_for_pid(pid);
}
