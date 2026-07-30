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

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;

#[cfg(feature = "nonos-dev-unverified-capsules")]
use crate::memory::addr::VirtAddr;

use super::core::{current_process, ProcessControlBlock, ProcessState, PROCESS_TABLE};
#[cfg(feature = "nonos-dev-unverified-capsules")]
use super::userspace::constants::{USER_STACK_BASE, USER_STACK_SIZE};

#[cfg(not(feature = "nonos-dev-unverified-capsules"))]
pub fn exec_process(
    _path: &str,
    _argv: &[String],
    _envp: &[String],
) -> Result<core::convert::Infallible, &'static str> {
    Err("exec_process is disabled; the only user-mode entry is capsule_spawn::spawn_verified")
}

#[cfg(feature = "nonos-dev-unverified-capsules")]
pub fn exec_process(
    path: &str,
    argv: &[String],
    envp: &[String],
) -> Result<core::convert::Infallible, &'static str> {
    exec_process_inner(path, argv, envp)
}

#[cfg(feature = "nonos-dev-unverified-capsules")]
fn exec_process_inner(
    path: &str,
    argv: &[String],
    envp: &[String],
) -> Result<core::convert::Infallible, &'static str> {
    let current = current_process().ok_or("no current process")?;

    // Activate the process's own address space before mapping the new
    // image. The ELF loader writes through the active address space,
    // so mappings have to land here, not in whatever was active before.
    if current.cr3.load(Ordering::Acquire) == 0 {
        return Err("process has no address space allocated");
    }
    crate::process::address_space::lifecycle::switch_to(current.pid)?;

    let executable_data = crate::fs::read_file(path)?;

    let elf_image = match crate::elf::loader::load_elf_executable(&executable_data) {
        Ok(img) => img,
        Err(_) => return Err("invalid executable format"),
    };

    if elf_image.entry_point.as_u64() == 0 {
        return Err("executable has no entry point");
    }

    {
        let mut mem = current.memory.lock();
        let total_pages: u64 =
            mem.vmas.iter().map(|vma| (vma.end.as_u64() - vma.start.as_u64()) / 4096).sum();
        mem.vmas.clear();
        mem.resident_pages.fetch_sub(total_pages, Ordering::Relaxed);
        mem.code_start = elf_image.base_addr;
        mem.code_end = elf_image.base_addr + elf_image.memory_size as u64;
        mem.next_va = 0x0000_4000_0000;
    }
    crate::process::address_space::lifecycle::record_segments(&current, &elf_image.segments);

    current.fd_table.close_cloexec();
    current.signals.lock().reset_for_exec();
    current.pending_signals.store(0, Ordering::Release);
    *current.name.lock() = path.into();
    {
        let mut a = current.argv.lock();
        a.clear();
        a.extend(argv.iter().cloned());
    }
    {
        let mut e = current.envp.lock();
        e.clear();
        e.extend(envp.iter().cloned());
    }

    let stack_top = VirtAddr::new(USER_STACK_BASE);
    crate::process::address_space::lifecycle::map_user_stack(&current, stack_top, USER_STACK_SIZE)?;

    let stack_config = crate::elf::stack::StackConfig::new()
        .with_args(argv.to_vec())
        .with_env(envp.to_vec())
        .with_stack_size(USER_STACK_SIZE);

    let stack_layout =
        match crate::elf::stack::setup_user_stack(stack_top, USER_STACK_SIZE, &stack_config) {
            Ok(layout) => layout,
            Err(_) => return Err("failed to setup user stack"),
        };

    let cr3 = current.cr3.load(Ordering::Acquire);
    if cr3 == 0 {
        return Err("no valid cr3 for process");
    }

    // User-space address ceiling on x86_64. Reject before iretq so a
    // malformed binary doesn't produce a hardware trap on entry.
    const USER_SPACE_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;
    let entry = elf_image.entry_point.as_u64();
    let stack = stack_layout.stack_pointer.as_u64();
    if entry > USER_SPACE_MAX {
        return Err("entry point not in user space");
    }
    if stack == 0 || stack > USER_SPACE_MAX {
        return Err("stack pointer not in user space");
    }

    let exec_ctx = super::userspace::types::ExecContext {
        entry,
        stack,
        pid: current.pid as u64,
        tid: current.pid as u64,
        cr3,
        argc: argv.len() as u64,
        argv: stack_layout.argv_ptr.as_u64(),
        envp: stack_layout.envp_ptr.as_u64(),
    };

    *current.state.lock() = ProcessState::Running;

    super::userspace::transitions::exec_process(&exec_ctx)
}

#[inline]
pub fn exec_fn(path: &str) -> Result<core::convert::Infallible, &'static str> {
    exec_process(path, &[], &[])
}

pub fn set_umask(mask: u32) -> u32 {
    let current = match current_process() {
        Some(p) => p,
        None => return 0o022,
    };

    let mut umask = current.umask.lock();
    let old_mask = *umask;
    *umask = mask;
    old_mask
}

pub fn set_root(path: &str) -> Result<(), &'static str> {
    let current = current_process().ok_or("no current process")?;

    if !crate::fs::is_directory(path) {
        return Err("not a directory");
    }

    let mut root = current.root_dir.lock();
    *root = String::from(path);
    Ok(())
}

pub fn update_memory_usage(process_id: u64, delta: i64) {
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(process_id as u32) {
        let memory = pcb.memory.lock();
        let pages = ((delta.unsigned_abs() + 4095) / 4096) as u64;

        if delta > 0 {
            memory.resident_pages.fetch_add(pages, Ordering::Relaxed);
        } else if delta < 0 {
            memory.resident_pages.fetch_sub(pages, Ordering::Relaxed);
        }

        let current_pages = memory.resident_pages.load(Ordering::Relaxed);
        if delta.abs() > 1024 * 1024 {
            crate::log::info!(
                "Process {} memory usage: {} pages (delta: {} bytes)",
                process_id,
                current_pages,
                delta
            );
        }
    }
}

pub fn exit_current_process(status: i32) -> ! {
    if let Some(pcb) = current_process() {
        pcb.on_thread_exit();
    }
    super::core::syscalls::sys_exit(status)
}

pub fn exit_thread(status: i32) -> ! {
    if let Some(pcb) = current_process() {
        pcb.on_thread_exit();

        let is_last_thread =
            pcb.thread_group.as_ref().map(|tg| tg.thread_count() <= 1).unwrap_or(true);

        if is_last_thread || pcb.is_group_leader() {
            super::core::syscalls::sys_exit(status);
        }

        pcb.exit_code.store(status, Ordering::Release);
        {
            let mut state = pcb.state.lock();
            *state = ProcessState::Terminated(status);
        }
    }

    crate::arch::halt_loop()
}

pub fn get_thread_count() -> u32 {
    current_process()
        .and_then(|pcb| pcb.thread_group.as_ref().map(|tg| tg.thread_count()))
        .unwrap_or(1)
}

pub fn get_thread_ids() -> Vec<u32> {
    current_process()
        .and_then(|pcb| pcb.thread_group.as_ref().map(|tg| tg.threads.read().clone()))
        .unwrap_or_else(|| current_process().map(|p| vec![p.pid]).unwrap_or_default())
}

#[inline]
pub fn get_current_process() -> Option<Arc<ProcessControlBlock>> {
    current_process()
}

pub fn enumerate_all_processes() -> Vec<super::types::Process> {
    super::core::get_process_table()
        .get_all_processes()
        .into_iter()
        .map(|pcb| {
            let name = pcb.name.lock().clone();
            super::types::Process::new(pcb.pid, name, Some(pcb))
        })
        .collect()
}

#[inline]
pub fn get_all_processes() -> Vec<super::types::Process> {
    enumerate_all_processes()
}
