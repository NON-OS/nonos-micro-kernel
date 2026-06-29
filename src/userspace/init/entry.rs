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

use super::{spawn_plan, supervisor::init_loop};
use crate::sys::boot_log;

pub fn run_init() -> ! {
    boot_log::ok("INIT", "Starting");
    run_user_entry_proof();
    run_std_proof();
    run_c_proof();
    run_ripgrep();
    spawn_plan::spawn_ramfs();
    spawn_plan::spawn_core_after_ramfs();
    spawn_plan::spawn_display_core();
    spawn_plan::spawn_drivers();
    spawn_plan::spawn_vfs();
    spawn_plan::spawn_network();
    spawn_plan::spawn_desktop();
    spawn_plan::spawn_market();
    spawn_plan::spawn_apps();
    boot_log::ok("INIT", "Capsules spawned");
    lower_init_priority();
    yield_after_spawns();
    launch_final_payload();
    init_loop()
}

#[cfg(feature = "nonos-user-entry-proof")]
fn run_user_entry_proof() {
    let _ = crate::userspace::capsule_proof_io::spawn_proof_io_capsule();
}

#[cfg(not(feature = "nonos-user-entry-proof"))]
fn run_user_entry_proof() {}

#[cfg(feature = "nonos-capsule-std-proof")]
fn run_std_proof() {
    match crate::userspace::capsule_std_proof::spawn_std_proof_capsule() {
        Ok(()) => boot_log::ok("STD-PROOF", "capsule spawned"),
        Err(_) => boot_log::error("STD-PROOF capsule spawn failed"),
    }
}

#[cfg(not(feature = "nonos-capsule-std-proof"))]
fn run_std_proof() {}

#[cfg(feature = "nonos-capsule-c-proof")]
fn run_c_proof() {
    match crate::userspace::capsule_c_proof::spawn_c_proof_capsule() {
        Ok(()) => boot_log::ok("C-PROOF", "capsule spawned"),
        Err(_) => boot_log::error("C-PROOF capsule spawn failed"),
    }
}

#[cfg(not(feature = "nonos-capsule-c-proof"))]
fn run_c_proof() {}

#[cfg(feature = "nonos-capsule-ripgrep")]
fn run_ripgrep() {
    match crate::userspace::capsule_ripgrep::spawn_ripgrep_capsule() {
        Ok(()) => boot_log::ok("RIPGREP", "capsule spawned"),
        Err(_) => boot_log::error("RIPGREP capsule spawn failed"),
    }
}

#[cfg(not(feature = "nonos-capsule-ripgrep"))]
fn run_ripgrep() {}

fn lower_init_priority() {
    use crate::process::core::{Priority, CURRENT_PID, PROCESS_TABLE};
    use core::sync::atomic::Ordering;
    let pid = CURRENT_PID.load(Ordering::Relaxed);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        *pcb.priority.lock() = Priority::Low;
    }
}

fn yield_after_spawns() {
    for _ in 0..100 {
        crate::sched::yield_now();
    }
}

fn launch_final_payload() {
    let _ = crate::userspace::capsule_proof_io::spawn_proof_io_capsule();
}
