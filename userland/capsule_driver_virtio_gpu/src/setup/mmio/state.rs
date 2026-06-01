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
use nonos_libc::{mk_mmio_unmap, mk_pio_release, MmioMapOut, PioGrantOut};
use crate::regs::Regs;

#[derive(Clone, Copy)]
pub struct ModernGrant {
    pub common: MmioMapOut,
    pub common_offset: usize,
    pub notify: MmioMapOut,
    pub notify_offset: usize,
    pub notify_multiplier: usize,
    pub device: MmioMapOut,
    pub device_offset: usize,
}

#[derive(Clone, Copy)]
pub enum RegisterGrant {
    Mmio(MmioMapOut),
    Pio(PioGrantOut),
    Modern(ModernGrant),
}
impl RegisterGrant {
    pub fn regs(self, _pci_device: u16) -> Regs {
        match self {
            Self::Mmio(g) => Regs::mmio(g.user_va),
            Self::Pio(g) => Regs::pio(g.grant_id),
            Self::Modern(g) => Regs::modern(
                g.common.user_va,
                g.common_offset,
                g.notify.user_va,
                g.notify_offset,
                g.notify_multiplier,
                g.device.user_va,
                g.device_offset,
            ),
        }
    }
    pub fn grant_id(self) -> u64 {
        match self {
            Self::Mmio(g) => g.grant_id,
            Self::Pio(g) => g.grant_id,
            Self::Modern(g) => g.common.grant_id,
        }
    }
    pub fn release(self) -> bool {
        match self {
            Self::Mmio(g) => mk_mmio_unmap(g.grant_id) >= 0,
            Self::Pio(g) => mk_pio_release(g.grant_id) >= 0,
            Self::Modern(g) => release_modern(g),
        }
    }
}

fn release_modern(g: ModernGrant) -> bool {
    let common = mk_mmio_unmap(g.common.grant_id);
    let notify = mk_mmio_unmap(g.notify.grant_id);
    let device = mk_mmio_unmap(g.device.grant_id);
    common >= 0 && notify >= 0 && device >= 0
}
