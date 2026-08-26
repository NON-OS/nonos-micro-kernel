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

#![no_std]

pub mod admin;
pub mod attest;
pub mod battery;
pub mod broker;
pub mod capsule_load;
pub mod capsule_verify;
pub mod crypto;
pub mod debug;
pub mod graphics;
#[cfg(feature = "heap")]
pub mod heap;
pub mod ipc;
pub mod mem;
#[cfg(feature = "panic-handler")]
mod panic;
pub mod proc_output;
pub mod process;
pub mod procstat;
pub mod spawn_instance;
pub mod store_write;
pub mod tool_run;
pub mod surface_registry;
mod syscall;
pub mod time;
pub mod transport;
mod unistd;

pub use admin::{mk_admin_policy_push, mk_admin_reboot, mk_admin_shutdown};
pub use attest::{mk_attest_status, AttestStatus};
pub use battery::mk_battery_status;
pub use broker::{
    mk_device_claim, mk_device_list, mk_device_release, mk_dma_map, mk_dma_unmap, mk_irq_ack,
    mk_irq_bind, mk_irq_poll, mk_irq_unbind, mk_irq_wait, mk_mmio_map, mk_mmio_unmap,
    mk_pci_config_read, mk_pci_config_write, mk_pio_grant, mk_pio_read, mk_pio_release,
    mk_pio_write, Bar, DeviceRecord, DmaMapOut, IrqBindOut, IrqPollOut, MmioMapOut, PioGrantOut,
    BAR_KIND_MMIO, BAR_KIND_NONE, BAR_KIND_PIO, BUS_KIND_ACPI, BUS_KIND_PCI, BUS_KIND_VIRT,
    MK_DMA_MAP_HIGH, MK_IRQ_BIND_MSIX, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
    MK_PCI_MSIX_CTRL_ENABLE, MK_PCI_MSIX_CTRL_FUNCTION_MASK,
};
pub use capsule_load::{mk_capsule_load, CapsuleLoadRequest};
pub use capsule_verify::{mk_capsule_verify, CapsuleVerifyRequest, CapsuleVerifySummary};
pub use spawn_instance::mk_spawn_instance;
pub use store_write::mk_store_write;
pub use tool_run::mk_tool_run;
pub use crypto::{
    crypto_decrypt, crypto_decrypt_aad, crypto_ed25519_pubkey, crypto_ed25519_sign, crypto_ed25519_verify, crypto_encrypt,
    crypto_encrypt_aad, crypto_hash, crypto_hkdf_sha256, crypto_hmac_sha256, crypto_keccak256,
    crypto_random, crypto_secp256k1_pubkey, crypto_secp256k1_sign, crypto_x25519_public,
    crypto_x25519_shared,
};
pub use debug::mk_debug;
pub use graphics::nonos_display_dimensions;
#[cfg(feature = "heap")]
pub use heap::{init as heap_init, init_sized as heap_init_sized, HeapError};
pub use ipc::{
    mk_ipc_call, mk_ipc_call_timeout, mk_ipc_recv, mk_ipc_recv_from, mk_ipc_reply, mk_ipc_send,
    mk_ipc_send_to_pid, mk_service_lookup, mk_service_register,
};
pub use mem::{mk_mmap, mk_munmap};
pub use proc_output::{mk_proc_input, mk_proc_output, mk_stdin_read};
pub use process::{mk_args, mk_getpid, mk_kill, mk_pid_alive, mk_wait};
pub use procstat::{mk_proc_stat, ProcStatEntry, ProcStatHeader, PROC_NAME_LEN};
pub use surface_registry::{
    mk_display_vsync_wait, mk_input_event_drain, mk_input_event_post, mk_input_event_wait,
    mk_surface_attach, mk_surface_present, mk_surface_present_rect, mk_surface_register,
    mk_surface_release, mk_surface_share, InputEvent, SurfaceDescriptor, INPUT_KIND_BUTTON_DOWN,
    INPUT_KIND_BUTTON_UP, INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP, INPUT_KIND_POINTER_ABS,
    INPUT_KIND_POINTER_REL, INPUT_KIND_TOUCH, INPUT_KIND_WHEEL, SURFACE_FORMAT_ARGB8888,
};
pub use syscall::call_raw as mk_syscall_raw;
pub use time::{mk_time_adjust, mk_time_millis, mk_time_rtc, mk_uptime_ms, Deadline, RtcTime};
pub use unistd::{mk_exit, mk_sleep_ms, mk_yield};
