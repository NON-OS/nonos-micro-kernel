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

extern crate alloc;

use super::super::super::spec::SpawnError;
use super::params::InstallParams;
use crate::capabilities::Capability;
use crate::ipc::nonos_inbox;
use crate::kernel_core::process_spawn::{
    allocate_kernel_stack, allocate_user_stack, setup_initial_user_context,
};
use crate::process::core::{create_process_with_parent, ProcessState};
use crate::services::registry::{register_endpoint, required_caps};
use alloc::format;

pub(crate) fn run(params: &InstallParams) -> Result<u32, SpawnError> {
    super::trace::trace(params.name, b"install enter");
    if params.elf.is_empty() {
        return Err(SpawnError::FeatureDisabled);
    }
    nonos_inbox::register_or_get_bootstrap_inbox(params.reply_inbox);
    register_endpoint(params.reply_inbox, params.reply_port, 0, 0).map_err(|_| {
        crate::sys::bench::mark_named(b"capsule_endpoint_collision", params.name.as_bytes());
        SpawnError::EndpointCollision
    })?;
    let pid = create_process_with_parent(
        params.name,
        ProcessState::Ready,
        super::priority::for_capsule(params.name),
        0,
        params.on_behalf_of,
    )
    .map_err(|_| SpawnError::ProcessCreation)?;
    crate::process::with_process(pid, |pcb| pcb.set_reply_inbox(params.reply_inbox))
        .ok_or(SpawnError::ProcessCreation)?;
    nonos_inbox::register_inbox(&format!("proc.{}", pid), pid)
        .map_err(|_| SpawnError::ProcessCreation)?;
    let entry = super::load_elf_into_pid::load_elf_into_pid(params.elf, pid, params.debug_tag)?;
    let caps = params.caps_bits;
    super::install_caps::install_caps(pid, caps)?;
    let _kernel_stack = allocate_kernel_stack(pid).map_err(|_| SpawnError::AddressSpace)?;
    let user_rsp = allocate_user_stack(pid).map_err(|_| SpawnError::AddressSpace)?;
    setup_initial_user_context(pid, entry, user_rsp).map_err(|_| SpawnError::AddressSpace)?;
    let service_caps = required_caps(params.name, Capability::IPC.bit());
    register_endpoint(params.name, params.service_port, pid, service_caps).map_err(|_| {
        crate::sys::bench::mark_named(b"capsule_endpoint_collision", params.name.as_bytes());
        SpawnError::EndpointCollision
    })?;
    super::spawn_log::emit(params.name, pid, caps, entry);
    crate::sched::add_to_run_queue(pid);
    crate::sys::bench::mark_named(b"capsule_runqueue_ok", params.name.as_bytes());
    super::trace::trace(params.name, b"runqueue ok");
    Ok(pid)
}
