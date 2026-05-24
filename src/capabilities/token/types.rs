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

use crate::capabilities::types::Capability;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct CapabilityToken {
    pub owner_module: u64,
    pub permissions: Vec<Capability>,
    pub expires_at_ms: Option<u64>,
    pub nonce: u64,
    pub signature: [u8; 64],
    // Authority material added in Phase 2 of the capability rewrite.
    // Populated on every mint; covered by the MAC. Enforcement lands
    // in a later step — this commit only carries the data.
    pub token_id: u64,
    pub subject_capsule_id: u32,
    pub subject_asid: u32,
    pub subject_measurement: [u8; 32],
    pub boot_session_nonce: [u8; 16],
    pub revocation_epoch: u64,
    pub delegation_depth: u8,
}

impl CapabilityToken {
    pub fn empty() -> Self {
        Self {
            owner_module: 0,
            permissions: Vec::new(),
            expires_at_ms: Some(0),
            nonce: 0,
            signature: [0u8; 64],
            token_id: 0,
            subject_capsule_id: 0,
            subject_asid: 0,
            subject_measurement: [0u8; 32],
            boot_session_nonce: [0u8; 16],
            revocation_epoch: 0,
            delegation_depth: 0,
        }
    }
    /// Test / fixture constructor. Binds no boot session nonce so it
    /// never accidentally lends an empty token live boot authority.
    pub fn with_caps(caps: &[Capability]) -> Self {
        Self {
            owner_module: 0,
            permissions: caps.to_vec(),
            expires_at_ms: None,
            nonce: 0,
            signature: [0u8; 64],
            token_id: 0,
            subject_capsule_id: 0,
            subject_asid: 0,
            subject_measurement: [0u8; 32],
            boot_session_nonce: [0u8; 16],
            revocation_epoch: 0,
            delegation_depth: 0,
        }
    }
    pub fn system() -> Self {
        use crate::capabilities::types::Capability::*;
        Self::with_caps(&[
            CoreExec,
            IO,
            FileSystem,
            Memory,
            Network,
            IPC,
            Crypto,
            Hardware,
            Debug,
            Admin,
            RegisterService,
        ])
    }
    #[inline]
    pub fn grants(&self, cap: Capability) -> bool {
        self.permissions.iter().any(|c| *c == cap)
    }
    #[inline]
    pub fn not_expired(&self) -> bool {
        self.expires_at_ms.map_or(true, |exp| crate::time::timestamp_millis() < exp)
    }
    pub fn remaining_ms(&self) -> Option<u64> {
        self.expires_at_ms.map(|exp| exp.saturating_sub(crate::time::timestamp_millis()))
    }
    pub fn permission_count(&self) -> usize {
        self.permissions.len()
    }
    pub fn has_any_permission(&self) -> bool {
        !self.permissions.is_empty()
    }
    pub fn grants_all(&self, caps: &[Capability]) -> bool {
        caps.iter().all(|c| self.grants(*c))
    }
    pub fn grants_any(&self, caps: &[Capability]) -> bool {
        caps.iter().any(|c| self.grants(*c))
    }
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.not_expired() && self.has_any_permission()
    }
    #[inline]
    pub fn is_admin(&self) -> bool {
        self.grants(Capability::Admin)
    }
    #[inline]
    pub fn can_register_service(&self) -> bool {
        self.grants(Capability::RegisterService)
    }
    #[inline]
    pub fn can_device_enum(&self) -> bool {
        self.grants(Capability::DeviceEnum) || self.grants(Capability::Admin)
    }
    // Claim / release authority. A driver capsule needs this on top
    // of `DeviceEnum` to take exclusive ownership of a device.
    #[inline]
    pub fn can_driver(&self) -> bool {
        self.grants(Capability::Driver) || self.grants(Capability::Admin)
    }
    // MMIO mapping authority. Required in addition to `Driver` —
    // claim ownership alone is not enough to receive a physical
    // mapping into the capsule's address space.
    #[inline]
    pub fn can_mmio(&self) -> bool {
        self.grants(Capability::Mmio) || self.grants(Capability::Admin)
    }
    // IRQ binding authority. Required in addition to `Driver` for
    // a capsule to receive interrupt delivery from a device it has
    // claimed.
    #[inline]
    pub fn can_irq(&self) -> bool {
        self.grants(Capability::Irq) || self.grants(Capability::Admin)
    }
    // DMA buffer authority. Required in addition to `Driver` for a
    // capsule to receive a DMA-coherent buffer the claimed device
    // can read or write through.
    #[inline]
    pub fn can_dma(&self) -> bool {
        self.grants(Capability::Dma) || self.grants(Capability::Admin)
    }
    // PIO grant authority. Required in addition to `Driver` for a
    // capsule to mint a port-window grant on a claimed device and
    // run kernel-mediated `in`/`out` instructions through it.
    #[inline]
    pub fn can_pio(&self) -> bool {
        self.grants(Capability::Pio) || self.grants(Capability::Admin)
    }
    #[inline]
    pub fn can_input_source(&self) -> bool {
        self.grants(Capability::InputSource)
            || self.grants(Capability::Irq)
            || self.grants(Capability::Admin)
    }
}

impl core::fmt::Display for CapabilityToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Token[owner:{} caps:{} nonce:{:016x}]",
            self.owner_module,
            self.permissions.len(),
            self.nonce
        )
    }
}
