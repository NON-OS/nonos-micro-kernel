// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use crate::process::Pid;

static PENDING: Mutex<Vec<Pid>> = Mutex::new(Vec::new());

pub(super) fn enqueue(pid: Pid) {
    let mut q = PENDING.lock();
    if !q.contains(&pid) {
        q.push(pid);
    }
}

pub(crate) fn drain() {
    let drained: Vec<Pid> = match PENDING.try_lock() {
        Some(mut q) => {
            if q.is_empty() {
                return;
            }
            q.drain(..).collect()
        }
        None => return,
    };
    for pid in drained {
        super::finalize::finalize_teardown(pid);
    }
}
