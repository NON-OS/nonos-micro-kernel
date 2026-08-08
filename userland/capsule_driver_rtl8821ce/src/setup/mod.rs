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

//! Claim the wifi device, enable bus mastering and map its register window
//! through the broker. The claim epoch and device id are carried out so the
//! later firmware download can map DMA memory against the same claim.

use nonos_libc::{
    mk_device_claim, mk_mmio_map, mk_pci_config_write, MmioMapOut, MK_PCI_CFG_COMMAND,
    MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

use core::sync::atomic::{AtomicU32, Ordering};

use crate::discover::{find, Found};
use crate::regs::Regs;

/// Which BAR the register window came from, and the low half of the address it
/// was mapped at. rtw88 hardcodes bar_id 2 for this chip; this driver takes the
/// first MMIO BAR the broker reports, so a change in what the broker reports
/// would silently move the window somewhere that still answers reads. On a
/// machine with no serial console the panel is the only place to check.
static BAR_INDEX: AtomicU32 = AtomicU32::new(0xFFFF_FFFF);
static WINDOW_VA: AtomicU32 = AtomicU32::new(0);

/// The BAR index the window was taken from and the low 32 bits of its address.
/// An index of `0xFFFFFFFF` means no window has been mapped.
pub fn window() -> (u32, u32) {
    (BAR_INDEX.load(Ordering::Relaxed), WINDOW_VA.load(Ordering::Relaxed))
}

pub struct Mapped {
    pub regs: Regs,
    pub device_id: u64,
    pub claim_epoch: u64,
    /// The board facts, read straight after power-on and before the firmware
    /// download and the MAC tables run. rtw88 reads the efuse at that point and
    /// only brings the MAC up afterwards; taking the read at the end instead found
    /// the efuse registers answering zero on real silicon, so it is taken here,
    /// while the chip is still in the state the reference driver reads it in.
    pub efuse: Option<crate::efuse::EfuseInfo>,
}

/// Find, claim and map the chip. Returns the register window ready for
/// power-on, or an error string for the on-screen status.
pub fn run() -> Result<Mapped, &'static str> {
    let dev: Found = find().ok_or("rtl8821ce: not present")?;
    let epoch = mk_device_claim(dev.device_id);
    if epoch <= 0 {
        return Err("rtl8821ce: claim failed");
    }
    // Enable memory space (so the MMIO BAR decodes) and bus mastering (so the
    // beacon queue can DMA staged firmware out of host memory). These are the
    // only two command bits the broker permits a driver to flip; the I/O-space
    // bit is neither needed (this driver uses the memory BAR, not the port BAR)
    // nor writable through that path, so it is deliberately left out.
    let command = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;
    if mk_pci_config_write(dev.device_id, epoch as u64, MK_PCI_CFG_COMMAND, command) < 0 {
        return Err("rtl8821ce: bus master enable failed");
    }
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let len = core::cmp::max(dev.bar_size, 0x1000);
    let r = mk_mmio_map(dev.device_id, epoch as u64, dev.bar_index, 0, 0, len, &mut out);
    if r < 0 {
        return Err("rtl8821ce: mmio map failed");
    }
    BAR_INDEX.store(dev.bar_index, Ordering::Relaxed);
    WINDOW_VA.store(out.user_va as u32, Ordering::Relaxed);
    Ok(Mapped {
        regs: Regs::new(out.user_va),
        device_id: dev.device_id,
        claim_epoch: epoch as u64,
        efuse: None,
    })
}
