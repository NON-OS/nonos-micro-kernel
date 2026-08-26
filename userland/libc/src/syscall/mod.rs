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

mod bridge;
mod numbers;
mod raw;

pub(crate) use bridge::call_diverging;
pub use bridge::call_raw;
pub(crate) use numbers::{
    N_ADMIN_POLICY_PUSH, N_ADMIN_REBOOT, N_ADMIN_SHUTDOWN, N_CRYPTO_DECRYPT,
    N_CRYPTO_DECRYPT_AAD, N_CRYPTO_ED25519_PUBKEY, N_CRYPTO_ED25519_SIGN,
    N_CRYPTO_ED25519_VERIFY, N_CRYPTO_ENCRYPT, N_CRYPTO_ENCRYPT_AAD, N_CRYPTO_HASH,
    N_CRYPTO_HKDF_SHA256, N_CRYPTO_HMAC_SHA256, N_CRYPTO_KECCAK256, N_CRYPTO_RANDOM,
    N_CRYPTO_SECP256K1_PUBKEY, N_CRYPTO_SECP256K1_SIGN, N_CRYPTO_X25519_PUBLIC,
    N_CRYPTO_X25519_SHARED, N_GFX_DISPLAY_DIMENSIONS, N_MK_ARGS, N_MK_ATTEST_STATUS,
    N_MK_BATTERY_STATUS, N_MK_CAPSULE_LOAD, N_MK_CAPSULE_VERIFY, N_MK_DEBUG, N_MK_DEVICE_CLAIM,
    N_MK_DEVICE_LIST, N_MK_DEVICE_RELEASE, N_MK_DISPLAY_VSYNC_WAIT, N_MK_DMA_MAP,
    N_MK_DMA_UNMAP, N_MK_EXIT, N_MK_GETPID, N_MK_INPUT_EVENT_DRAIN, N_MK_INPUT_EVENT_POST,
    N_MK_INPUT_EVENT_WAIT, N_MK_IPC_CALL, N_MK_IPC_RECV, N_MK_IPC_RECV_FROM, N_MK_IPC_REPLY,
    N_MK_IPC_SEND, N_MK_IPC_SEND_TO_PID, N_MK_IRQ_ACK, N_MK_IRQ_BIND, N_MK_IRQ_POLL,
    N_MK_IRQ_UNBIND, N_MK_IRQ_WAIT, N_MK_KILL, N_MK_MMAP, N_MK_MMIO_MAP, N_MK_MMIO_UNMAP,
    N_MK_PCI_CONFIG_READ, N_MK_PCI_CONFIG_WRITE, N_MK_PID_ALIVE, N_MK_PIO_GRANT, N_MK_PIO_READ,
    N_MK_PIO_RELEASE, N_MK_PIO_WRITE, N_MK_PROC_INPUT, N_MK_PROC_OUTPUT, N_MK_PROC_STAT,
    N_MK_SERVICE_LOOKUP, N_MK_SERVICE_REGISTER, N_MK_SLEEP_MS, N_MK_SPAWN_INSTANCE,
    N_MK_STDIN_READ, N_MK_STORE_WRITE, N_MK_SURFACE_ATTACH, N_MK_SURFACE_PRESENT,
    N_MK_SURFACE_REGISTER, N_MK_SURFACE_RELEASE, N_MK_SURFACE_SHARE, N_MK_TIME_ADJUST,
    N_MK_TIME_MILLIS, N_MK_TIME_MONOTONIC, N_MK_TIME_RTC, N_MK_TOOL_RUN, N_MK_WAIT, N_MK_YIELD,
};
