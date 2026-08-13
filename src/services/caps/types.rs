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

// CAP_VFS/NET/DISPLAY must equal the kernel Capability bits, not an independent
// 1<<N. As 1<<0/1/2 they aliased ambient or unrelated bits -- 1<<0 is the
// AMBIENT CoreExec every capsule holds, so the VFS gate collapsed to "any
// capsule" -- and these service gates never enforced their intended
// FileSystem/Network/Display authority. Binding to bit() closes the aliases;
// the capsule manifests that reach these gates carry the matching cap.
pub const CAP_VFS: u64 = crate::capabilities::Capability::FileSystem.bit();
pub const CAP_NET: u64 = crate::capabilities::Capability::Network.bit();
pub const CAP_DISPLAY: u64 = crate::capabilities::Capability::GraphicsDisplayQuery.bit();
// CAP_DRIVER must equal the kernel Capability::Driver bit, not an independent
// 1<<3. These service constants are tested against the PCB capability bitmap
// (services::caps::has -> process::caps::has), which is built from
// Capability::bit(). 1<<3 aliases Capability::IPC, an AMBIENT bit every capsule
// holds, so the driver gates in src/hardware/*_capsule collapsed to "any
// capsule". Binding to the real Driver bit makes them require the same
// authority the broker's MkDeviceClaim already demands (can_driver), which every
// legitimate driver capsule holds.
pub const CAP_DRIVER: u64 = crate::capabilities::Capability::Driver.bit();
// CAP_CRYPTO must equal the kernel Crypto bit, not 1<<4 which aliases the
// AMBIENT Memory bit. The kernel-side crypto client gate is reached from the
// crypto syscall dispatch, which already requires Capability::Crypto, so every
// caller holds it and this closes the open alias without locking anyone out.
pub const CAP_CRYPTO: u64 = crate::capabilities::Capability::Crypto.bit();
pub const CAP_PROCESS: u64 = 1 << 5;
pub const CAP_MEMORY: u64 = 1 << 6;
pub const CAP_INPUT: u64 = 1 << 7;
pub const CAP_AUDIO: u64 = 1 << 8;
pub const CAP_ZK: u64 = 1 << 9;
pub const CAP_GPU: u64 = 1 << 10;
// CAP_APPS must equal the kernel AppInstall bit, not 1<<11 which is
// GraphicsDisplayQuery. Every capsule that draws a window holds that, so the
// marketplace gate was open to all of them.
pub const CAP_APPS: u64 = crate::capabilities::Capability::AppInstall.bit();
pub const CAP_AGENTS: u64 = 1 << 12;
pub const CAP_SHELL: u64 = 1 << 13;
pub const CAP_KERNEL: u64 = 1 << 14;
// CAP_ENTROPY must equal the kernel Entropy bit, not 1<<15 which is DeviceEnum.
// The entropy read gate therefore admitted anything that could enumerate
// devices, which is every driver capsule and the broker.
pub const CAP_ENTROPY: u64 = crate::capabilities::Capability::Entropy.bit();
// CAP_KEYRING must equal the kernel Keyring bit, not 1<<16 which is Driver.
// Every driver capsule holds Driver, since CAP_DRIVER is bound to the real bit,
// so the keyring gate admitted all of them to key material.
pub const CAP_KEYRING: u64 = crate::capabilities::Capability::Keyring.bit();
// CAP_STORAGE must equal the kernel StoreWrite bit, not an independent 1<<17
// that no PCB ever carries. It gates the block surface for the runtime store
// path (sys_store_write runs in the caller's context), so it has to match the
// same authority vfs already holds to persist installed capsules.
pub const CAP_STORAGE: u64 = crate::capabilities::Capability::StoreWrite.bit();
pub const CAP_UDEV: u64 = 1 << 18;
pub const CAP_WALLET: u64 = 1 << 19;
pub const CAP_TLS: u64 = 1 << 20;
// CAP_ADMIN must equal the kernel Admin bit. At 1<<63 it sat above every bit a
// PCB can carry, so the gate it guards, entropy reseed, could not be passed by
// anything and the path was unreachable rather than restricted.
pub const CAP_ADMIN: u64 = crate::capabilities::Capability::Admin.bit();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceCap {
    pub bits: u64,
    pub owner_pid: u32,
    pub expires_ms: u64,
}

impl ServiceCap {
    pub fn new(bits: u64, owner: u32) -> Self {
        Self { bits, owner_pid: owner, expires_ms: 0 }
    }

    pub fn with_expiry(bits: u64, owner: u32, expires_ms: u64) -> Self {
        Self { bits, owner_pid: owner, expires_ms }
    }

    pub fn has(&self, cap: u64) -> bool {
        (self.bits & cap) == cap
    }

    pub fn is_expired(&self, now_ms: u64) -> bool {
        self.expires_ms != 0 && now_ms > self.expires_ms
    }
}
