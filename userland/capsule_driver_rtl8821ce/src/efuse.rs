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

//! Read the board facts the PHY needs out of the efuse. The RF front-end option
//! and the package type are burned per module and are not autoloaded into any
//! MAC register, so the PHY cannot be configured without them: the wrong RF
//! front-end selects the wrong radio tables. This reimplements the two-step
//! rtw88 read: dump the physical efuse a byte at a time through the efuse control
//! register, then de-shuffle its packed block/word format into the logical map
//! the option byte is indexed from. The 8821c defines no efuse-access grant op,
//! so only the bank select and the 2.5V LDO disable bracket the read. Values are
//! rtw88 facts (`reg.h`, `efuse.c`, `rtw8821c.c`), reimplemented not copied.

use core::sync::atomic::{AtomicU32, Ordering};

use crate::regs::Mmio;

/// The efuse control register: address in bits 8..18, data in the low byte, and
/// bit 31 the read-complete flag.
const REG_EFUSE_CTRL: usize = 0x0030;
/// The LDO/efuse control register: its bank-select bits and, at byte 3, the 2.5V
/// LDO enable.
const REG_LDO_EFUSE_CTRL: usize = 0x0034;
/// The chip configuration register whose bits 12..16 carry the cut version.
const REG_CHIP_VER: usize = 0x00F0;

const EF_ADDR_MASK: u32 = 0x3ff;
const EF_ADDR_SHIFT: u32 = 8;
const EF_DATA_MASK: u32 = 0xff;
const EF_FLAG: u32 = 1 << 31;
/// Bank select bits 8..9 of the LDO/efuse control; zero selects the wifi bank.
const BANK_SEL_MASK: u32 = (1 << 8) | (1 << 9);
/// Bit 7 of `REG_LDO_EFUSE_CTRL + 3`: the 2.5V LDO enable, cleared for the read.
const LDO25_EN: u8 = 1 << 7;

/// The physical and logical efuse are both 512 bytes on the 8821c.
const EFUSE_SIZE: usize = 512;
/// The logical offset of the RF front-end option byte.
const RFE_OPTION_OFFSET: usize = 0xCA;
/// The logical offset of the six-byte station MAC address (PCIe efuse layout,
/// rtw88 `struct rtw8821ce_efuse.mac_addr`).
const MAC_ADDR_OFFSET: usize = 0xD0;

/// What the chip said when an efuse read stalled: the control register as it last
/// read back, the byte address it stalled on, and the LDO/bank register. A stage
/// name alone cannot distinguish a dead register window from a live one whose
/// efuse controller is not clocked, and this machine has no serial console, so
/// these travel to the panel in the status reply.
static LAST_CTL: AtomicU32 = AtomicU32::new(0);
static LAST_ADDR: AtomicU32 = AtomicU32::new(0);
static LAST_LDO: AtomicU32 = AtomicU32::new(0);

/// The control, address and LDO registers recorded by the last stalled read.
/// All zero means no read has stalled since boot.
pub fn diag() -> (u32, u32, u32) {
    (
        LAST_CTL.load(Ordering::Relaxed),
        LAST_ADDR.load(Ordering::Relaxed),
        LAST_LDO.load(Ordering::Relaxed),
    )
}

/// The board facts the PHY bring-up needs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EfuseInfo {
    /// The cut (chip revision) version from the config register.
    pub cut: u8,
    /// The RF front-end option, five bits, selecting the radio tables.
    pub rfe: u8,
    /// The package type, one bit alongside the RF front-end option.
    pub pkg: u8,
    /// The station MAC address burned into the efuse. The chip does NOT autoload
    /// it into the MAC-ID registers on this board, so the driver must read it here
    /// and program the registers itself; otherwise the station runs on an all-zero
    /// MAC, which associates but leaves the access point unable to address the
    /// unicast EAPOL handshake, so the four-way never completes.
    pub mac: [u8; 6],
}

/// Read the efuse and the cut version and extract the PHY board facts, or `None`
/// if the efuse never completed a byte read.
pub fn read<M: Mmio>(mmio: &M) -> Option<EfuseInfo> {
    let phys = dump_physical(mmio)?;
    // A window that reads back all-ones sets the ready flag on every read, so
    // every byte "completes" instantly and the poll never stalls: an unreadable
    // efuse is otherwise indistinguishable from a blank one. Programming the RF
    // from that map picks a front-end the chip does not have.
    if phys.iter().all(|&b| b == 0xFF) {
        LAST_CTL.store(0xFFFF_FFFF, Ordering::Relaxed);
        LAST_ADDR.store(1, Ordering::Relaxed);
        return None;
    }
    let mut log = [0xffu8; EFUSE_SIZE];
    deshuffle(&phys, &mut log);
    let opt = log[RFE_OPTION_OFFSET];
    // 0xFF is the erased value, so an option byte that reads erased was never
    // burned and there is no front-end to select. The tables index off this.
    if opt == 0xFF {
        return None;
    }
    let cut = ((mmio.read32(REG_CHIP_VER) >> 12) & 0xf) as u8;
    let mut mac = [0u8; 6];
    mac.copy_from_slice(&log[MAC_ADDR_OFFSET..MAC_ADDR_OFFSET + 6]);
    Some(EfuseInfo { cut, rfe: opt & 0x1f, pkg: (opt >> 5) & 1, mac })
}

// Read the whole physical efuse a byte at a time. Select the wifi bank and drop
// the 2.5V LDO first, then for each address clear the flag to start a read and
// spin until the hardware sets it back with the byte in the low bits.
pub(crate) fn dump_physical<M: Mmio>(mmio: &M) -> Option<[u8; EFUSE_SIZE]> {
    let bank = mmio.read32(REG_LDO_EFUSE_CTRL) & !BANK_SEL_MASK;
    mmio.write32(REG_LDO_EFUSE_CTRL, bank);
    let ldo = mmio.read8(REG_LDO_EFUSE_CTRL + 3) & !LDO25_EN;
    mmio.write8(REG_LDO_EFUSE_CTRL + 3, ldo);

    let mut map = [0u8; EFUSE_SIZE];
    let mut ctl = mmio.read32(REG_EFUSE_CTRL);
    for (addr, slot) in map.iter_mut().enumerate() {
        ctl &= !(EF_DATA_MASK | (EF_ADDR_MASK << EF_ADDR_SHIFT));
        ctl |= (addr as u32 & EF_ADDR_MASK) << EF_ADDR_SHIFT;
        mmio.write32(REG_EFUSE_CTRL, ctl & !EF_FLAG);

        let mut spins = 1_000_000u32;
        loop {
            ctl = mmio.read32(REG_EFUSE_CTRL);
            if ctl & EF_FLAG != 0 {
                break;
            }
            spins -= 1;
            if spins == 0 {
                LAST_CTL.store(ctl, Ordering::Relaxed);
                // Stored one past the address so zero keeps meaning "no read has
                // stalled". A window that reads back all-zero would otherwise be
                // indistinguishable from a loop that was never entered, and those
                // are opposite faults.
                LAST_ADDR.store(addr as u32 + 1, Ordering::Relaxed);
                LAST_LDO.store(mmio.read32(REG_LDO_EFUSE_CTRL), Ordering::Relaxed);
                return None;
            }
        }
        *slot = (ctl & EF_DATA_MASK) as u8;
    }
    Some(map)
}

// Unpack the physical efuse into the logical map. The physical efuse is a run of
// blocks: a one- or two-byte header names the block and which of its four words
// are present, then those words follow. A word-enable bit set means the word was
// never written and is skipped, leaving the 0xff the caller pre-filled.
pub(crate) fn deshuffle(phys: &[u8; EFUSE_SIZE], log: &mut [u8; EFUSE_SIZE]) {
    let mut i = 0usize;
    while i + 1 < EFUSE_SIZE {
        let hdr1 = phys[i];
        let hdr2 = phys[i + 1];
        // An erased header (or an erased second byte of a two-byte header) ends
        // the map.
        if hdr1 == 0xff || ((hdr1 & 0x1f) == 0x0f && hdr2 == 0xff) {
            break;
        }

        let blk_idx;
        let word_en;
        if (hdr1 & 0x1f) == 0x0f {
            blk_idx = ((((hdr2 & 0xf0) >> 1) | ((hdr1 >> 5) & 0x07)) as usize) << 3;
            word_en = hdr2 & 0x0f;
            i += 2;
        } else {
            blk_idx = (((hdr1 & 0xf0) >> 4) as usize) << 3;
            word_en = hdr1 & 0x0f;
            i += 1;
        }

        for w in 0..4 {
            if word_en & (1 << w) != 0 {
                continue;
            }
            let log_idx = blk_idx + (w << 1);
            if i + 1 >= EFUSE_SIZE || log_idx + 1 >= EFUSE_SIZE {
                return;
            }
            log[log_idx] = phys[i];
            log[log_idx + 1] = phys[i + 1];
            i += 2;
        }
    }
}
