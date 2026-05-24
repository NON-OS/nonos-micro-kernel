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
use crate::discover::Found;
use crate::regs::Regs;
const PAGE_MASK: u64 = 0xFFF;
#[derive(Clone, Copy)]
pub enum RegisterGrant {
    Mmio(MmioMapOut),
    Pio(PioGrantOut),
}
impl RegisterGrant {
    pub fn regs(self) -> Regs {
        match self {
            Self::Mmio(g) => Regs::mmio(g.user_va),
            Self::Pio(g) => Regs::pio(g.grant_id),
        }
    }
    pub fn release(self) -> bool {
        match self {
            Self::Mmio(g) => mk_mmio_unmap(g.grant_id) >= 0,
            Self::Pio(g) => mk_pio_release(g.grant_id) >= 0,
        }
    }
}
pub fn grant(dev: Found, epoch: u64) -> Result<RegisterGrant, &'static str> {
    match dev.register_kind {
        BAR_KIND_MMIO => grant_mmio(dev, epoch),
        BAR_KIND_PIO => grant_pio(dev, epoch),
        _ => Err("virtio-blk: unsupported register bar"),
    }
}
fn grant_mmio(dev: Found, epoch: u64) -> Result<RegisterGrant, &'static str> {
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let length = (dev.register_size + PAGE_MASK) & !PAGE_MASK;
    let r = mk_mmio_map(dev.device_id, epoch, dev.register_bar as u32, 0, 0, length, &mut out);
    if r < 0 {
        if mk_device_release(dev.device_id) < 0 {
            return Err("device release failed after mmio map failure");
        }
        return Err("mmio map failed");
    }
    Ok(RegisterGrant::Mmio(out))
}
fn grant_pio(dev: Found, epoch: u64) -> Result<RegisterGrant, &'static str> {
    let mut out = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    let r = mk_pio_grant(dev.device_id, epoch, dev.register_bar, 0, &mut out);
    if r < 0 {
        if mk_device_release(dev.device_id) < 0 {
            return Err("device release failed after pio grant failure");
        }
        return Err("pio grant failed");
    }
    Ok(RegisterGrant::Pio(out))
}
