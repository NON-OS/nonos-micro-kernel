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

use super::CapabilityToken;
use crate::capabilities::types::Capability;

impl CapabilityToken {
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
