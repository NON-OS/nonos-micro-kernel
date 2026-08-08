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

pub mod attest;
pub mod battery;
pub mod capability;
pub mod capsule_load;
pub mod capsule_verify;
pub mod debug;
#[cfg(feature = "nonos-user-entry-proof")]
pub mod debug_diag;
pub mod device;
pub mod dispatch;
#[cfg(feature = "nonos-user-entry-proof")]
pub mod dispatch_trace;
pub mod dma;
pub mod errnos;
pub mod futex;
pub mod ipc;
pub mod irq;
pub mod kill;
pub mod memory;
pub mod mmio;
pub mod numbers;
pub mod pci;
pub mod pio;
pub mod proc_output;
pub mod proc_stdin;
pub mod process;
pub mod procstat;
pub mod spawn_instance;
pub mod stdout_write;
pub mod store_write;
pub mod time;
pub mod tool_run;
pub mod wait;

pub use attest::sys_attest_status;
pub use battery::sys_battery_status;
pub use capability::{sys_cap_check, sys_cap_grant, sys_cap_revoke};
pub use debug::sys_mk_debug;
pub use device::{sys_device_claim, sys_device_list, sys_device_release};
pub use dispatch::dispatch_microkernel_syscall;
pub use dma::{sys_dma_map, sys_dma_unmap};
pub use futex::{sys_futex_wait, sys_futex_wake};
pub use ipc::{sys_ipc_call, sys_ipc_recv, sys_ipc_reply, sys_ipc_send};
pub use irq::{sys_irq_ack, sys_irq_bind, sys_irq_poll, sys_irq_unbind};
pub use kill::sys_kill;
pub use memory::{sys_mmap, sys_munmap};
pub use mmio::{sys_mmio_map, sys_mmio_unmap};
pub use numbers::*;
pub use pci::sys_pci_config_write;
pub use pio::{sys_pio_grant, sys_pio_read, sys_pio_release, sys_pio_write};
pub use proc_output::sys_proc_output;
pub use proc_stdin::{sys_proc_input, sys_stdin_read};
pub use process::{sys_args, sys_exit, sys_set_tls, sys_spawn, sys_thread_spawn, sys_yield};
pub use procstat::sys_proc_stat;
pub use stdout_write::sys_stdout_write;
pub use store_write::sys_store_write;
pub use time::{sys_time_adjust, sys_time_millis, sys_time_monotonic, sys_time_rtc};
pub use wait::sys_wait;
