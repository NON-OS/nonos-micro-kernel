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
use super::super::spec::SpawnError;
use crate::capabilities::Capability;
use crate::elf::loader::load_elf_entry_into;
use crate::ipc::nonos_inbox;
use crate::kernel_core::process_spawn::{
    allocate_kernel_stack, allocate_user_stack, setup_initial_user_context,
};
use crate::memory::paging::manager::lookup_asid_for_process;
use crate::process::caps as proc_caps;
use crate::process::core::{create_process, Priority, ProcessState};
use crate::services::registry::register_endpoint;
use alloc::format;

pub(super) struct InstallParams {
    pub name: &'static str,
    pub service_port: u32,
    pub reply_inbox: &'static str,
    pub reply_port: u32,
    pub elf: &'static [u8],
    pub caps_bits: u64,
    pub debug_tag: &'static [u8],
}

pub(super) fn install(params: &InstallParams) -> Result<u32, SpawnError> {
    trace(params.name, b"install enter");
    if params.elf.is_empty() {
        return Err(SpawnError::FeatureDisabled);
    }

    nonos_inbox::register_or_get_bootstrap_inbox(params.reply_inbox);
    trace(params.name, b"reply inbox ok");
    register_endpoint(params.reply_inbox, params.reply_port, 0, 0)
        .map_err(|_| SpawnError::EndpointCollision)?;
    trace(params.name, b"reply endpoint ok");

    let pid = create_process(params.name, ProcessState::Ready, Priority::Normal)
        .map_err(|_| SpawnError::ProcessCreation)?;
    trace(params.name, b"process ok");

    crate::process::with_process(pid, |pcb| pcb.set_reply_inbox(params.reply_inbox))
        .ok_or(SpawnError::ProcessCreation)?;
    trace(params.name, b"pcb reply ok");

    nonos_inbox::register_inbox(&format!("proc.{}", pid), pid)
        .map_err(|_| SpawnError::ProcessCreation)?;
    trace(params.name, b"proc inbox ok");

    let entry = load_elf_into_pid(params.elf, pid, params.debug_tag)?;
    trace(params.name, b"elf ok");

    install_caps(pid, params.caps_bits | crate::capabilities::smoke::debug_grant())?;
    trace(params.name, b"caps ok");

    let _kernel_stack = allocate_kernel_stack(pid).map_err(|_| SpawnError::AddressSpace)?;
    trace(params.name, b"kstack ok");
    let user_rsp = allocate_user_stack(pid).map_err(|_| SpawnError::AddressSpace)?;
    trace(params.name, b"ustack ok");
    setup_initial_user_context(pid, entry, user_rsp).map_err(|_| SpawnError::AddressSpace)?;
    trace(params.name, b"context ok");

    register_endpoint(params.name, params.service_port, pid, Capability::IPC.bit())
        .map_err(|_| SpawnError::EndpointCollision)?;
    trace(params.name, b"service ok");

    crate::sched::add_to_run_queue(pid);
    trace(params.name, b"runqueue ok");
    Ok(pid)
}

fn load_elf_into_pid(
    elf: &'static [u8],
    pid: u32,
    debug_tag: &'static [u8],
) -> Result<u64, SpawnError> {
    let asid = lookup_asid_for_process(pid).ok_or(SpawnError::AddressSpace)?;
    load_elf_entry_into(elf, asid).map_err(|err| {
        crate::sys::serial::println(debug_tag);
        crate::sys::serial::println(err.as_str().as_bytes());
        SpawnError::ElfLoad
    })
}

fn install_caps(pid: u32, caps_bits: u64) -> Result<(), SpawnError> {
    proc_caps::install_spawn(pid, caps_bits).ok_or(SpawnError::ProcessCreation)
}

fn trace(name: &str, label: &[u8]) {
    if !matches!(
        name,
        "clipboard" | "login" | "toolkit" | "driver.virtio_net0" | "driver.virtio_gpu0"
    ) {
        return;
    }
    crate::sys::serial::print(b"[SPAWN] ");
    crate::sys::serial::print(name.as_bytes());
    crate::sys::serial::print(b" ");
    crate::sys::serial::println(label);
}
