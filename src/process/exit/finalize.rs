// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use crate::process::core::{Pid, PROCESS_TABLE};

pub(super) fn finalize_teardown(pid: Pid) {
    let pcb = match PROCESS_TABLE.find_by_pid(pid) {
        Some(p) => p,
        None => return,
    };

    crate::process::address_space::lifecycle::release(&pcb);
    let _ = crate::hardware::broker::release_all_for_pid(pid, false);
    let _ = crate::hardware::broker::irq_release_all_for_pid(pid);
    let _ = crate::hardware::broker::dma_release_all_for_pid(pid, false);
    #[cfg(target_arch = "x86_64")]
    let _ = crate::hardware::broker::pio_release_all_for_pid(pid);
    let _ = crate::services::registry::unregister_endpoints_for_pid(pid);
    // The reply endpoint is registered kernel-owned (pid 0), so the per-pid
    // sweep above misses it. Drop it by name and remove its inbox, or an
    // on-demand window instance that closes would leak its reply slot and the
    // next spawn into that slot would collide on the stale endpoint.
    if let Some(reply) = *pcb.reply_inbox.read() {
        let _ = crate::services::registry::unregister_endpoint_by_name(reply);
        let _ = crate::ipc::nonos_inbox::unregister_inbox(reply);
    }
    if !super::postmortem::is_retained(pid) {
        let _ = crate::ipc::nonos_inbox::unregister_for_pid(pid);
    }

    crate::process::clear_interrupt_context(pid);
    crate::process::clear_fpu_state(pid);
    crate::process::core::init::reparent_orphans(pid);
    let _ = PROCESS_TABLE.terminate_process(pid);
}
