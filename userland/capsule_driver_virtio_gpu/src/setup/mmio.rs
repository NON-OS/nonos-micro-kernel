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

use nonos_libc::{
    mk_device_release, mk_mmio_map, mk_mmio_unmap, mk_pio_grant, mk_pio_release, MmioMapOut,
    PioGrantOut, BAR_KIND_MMIO, BAR_KIND_PIO,
};

use crate::constants::BAR_OFFSET;
use crate::discover::Found;
use crate::regs::Regs;

const PAGE_MASK: u64 = 0xFFF;

#[derive(Clone, Copy)]
pub enum RegisterGrant {
    Mmio(MmioMapOut),
    Pio(PioGrantOut),
}

impl RegisterGrant {
    pub fn regs(self, dev: Found) -> Regs {
        match self {
            Self::Mmio(g) if dev.has_virtio => Regs::mmio_with_notify(
                g.user_va + dev.common_off as u64,
                dev.notify_rel as usize,
            ),
            Self::Mmio(g) => Regs::mmio(g.user_va),
            Self::Pio(g) => Regs::pio(g.grant_id),
        }
    }

    pub fn grant_id(self) -> u64 {
        match self {
            Self::Mmio(g) => g.grant_id,
            Self::Pio(g) => g.grant_id,
        }
    }

    pub fn release(self) {
        match self {
            Self::Mmio(g) => {
                let _ = mk_mmio_unmap(g.grant_id);
            }
            Self::Pio(g) => {
                let _ = mk_pio_release(g.grant_id);
            }
        }
    }
}

pub fn grant(dev: Found, claim_epoch: u64) -> Result<RegisterGrant, &'static str> {
    match dev.register_kind {
        BAR_KIND_MMIO => map_mmio(dev, claim_epoch),
        BAR_KIND_PIO => grant_pio(dev, claim_epoch),
        _ => Err("virtio-gpu: unsupported register bar"),
    }
}

fn map_mmio(dev: Found, claim_epoch: u64) -> Result<RegisterGrant, &'static str> {
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let length = (dev.register_size + PAGE_MASK) & !PAGE_MASK;
    let r = mk_mmio_map(
        dev.device_id,
        claim_epoch,
        dev.register_bar as u32,
        0,
        BAR_OFFSET,
        length,
        &mut out,
    );
    if r < 0 {
        let _ = mk_device_release(dev.device_id);
        Err(errno_label(r))
    } else {
        Ok(RegisterGrant::Mmio(out))
    }
}

fn grant_pio(dev: Found, claim_epoch: u64) -> Result<RegisterGrant, &'static str> {
    let mut out = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    let r = mk_pio_grant(dev.device_id, claim_epoch, dev.register_bar, 0, &mut out);
    if r < 0 {
        let _ = mk_device_release(dev.device_id);
        Err(pio_errno_label(r))
    } else {
        Ok(RegisterGrant::Pio(out))
    }
}

fn errno_label(errno: i64) -> &'static str {
    match errno {
        -1 => "virtio-gpu: mmio map denied",
        -12 => "virtio-gpu: mmio map no memory",
        -19 => "virtio-gpu: mmio map no device",
        -22 => "virtio-gpu: mmio map invalid range",
        -95 => "virtio-gpu: mmio map unsupported flags",
        -116 => "virtio-gpu: mmio map stale claim",
        _ => "virtio-gpu: mmio map failed",
    }
}

fn pio_errno_label(errno: i64) -> &'static str {
    match errno {
        -1 => "virtio-gpu: pio grant denied",
        -12 => "virtio-gpu: pio grant no memory",
        -19 => "virtio-gpu: pio grant no device",
        -22 => "virtio-gpu: pio grant invalid range",
        -95 => "virtio-gpu: pio grant unsupported flags",
        -116 => "virtio-gpu: pio grant stale claim",
        _ => "virtio-gpu: pio grant failed",
    }
}
