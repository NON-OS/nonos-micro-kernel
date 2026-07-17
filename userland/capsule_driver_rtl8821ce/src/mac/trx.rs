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

//! Transmit/receive configuration: the step that actually switches the MAC's
//! DMA and receive engines on. This is rtw88 `rtw_init_trx_cfg` for the 8821c on
//! PCIe (`mac.c` `txdma_queue_mapping`, `priority_queue_cfg` and `init_h2c`),
//! reimplemented register-for-register. Without it the register table configures
//! the receiver but never enables it, so the radio is fully brought up yet hears
//! nothing. The page counts and reserved-page boundary are derived from the
//! chip's FIFO sizes exactly as rtw88 derives them, so they read as arithmetic
//! rather than magic. The whole program is checked against a modeled device in
//! `rtl8821ce_proofs`.

use crate::regs::Mmio;

// Register offsets (rtw88 reg.h).
const REG_CR: usize = 0x0100;
const REG_TXDMA_PQ_MAP: usize = 0x010C;
const REG_RXFF_BNDY: usize = 0x011C;
const REG_FIFOPAGE_CTRL_2: usize = 0x0204;
const REG_AUTO_LLT_V1: usize = 0x0208;
const REG_TXDMA_OFFSET_CHK: usize = 0x020C;
const REG_RQPN_CTRL_2: usize = 0x022C;
const REG_FIFOPAGE_INFO_1: usize = 0x0230;
const REG_FIFOPAGE_INFO_2: usize = 0x0234;
const REG_FIFOPAGE_INFO_3: usize = 0x0238;
const REG_FIFOPAGE_INFO_4: usize = 0x023C;
const REG_FIFOPAGE_INFO_5: usize = 0x0240;
const REG_H2C_HEAD: usize = 0x0244;
const REG_H2C_TAIL: usize = 0x0248;
const REG_H2C_READ_ADDR: usize = 0x024C;
const REG_H2C_INFO: usize = 0x0254;
const REG_FWHW_TXQ_CTRL: usize = 0x0420;
const REG_BCNQ_BDNY_V1: usize = 0x0424;
const REG_BCNQ1_BDNY_V1: usize = 0x0456;
const REG_H2CQ_CSR: usize = 0x1330;
const REG_H2C_PKT_READADDR: usize = 0x10D0;
const REG_H2C_PKT_WRITEADDR: usize = 0x10D4;

// PCI transmit/receive DMA control (rtw88 pci.h). Resetting the DMA interface and
// enabling the receive tag is what makes the card start advancing the receive
// write index; clearing the ring pointers and the H2C index gives it a clean
// start.
const REG_PCI_CTRL: usize = 0x0300;
const RST_TRXDMA_INTF: u32 = 1 << 20;
const RX_TAG_EN: u32 = 1 << 15;
const REG_TXBD_RWPTR_CLR: usize = 0x039C;
const CLR_H2CQ_HOST_IDX: u32 = 1 << 16;
const CLR_H2CQ_HW_IDX: u32 = 1 << 8;

// All eight engine-enable bits: HCI TX/RX DMA, TX/RX DMA, protocol, schedule,
// MAC TX/RX. Writing this to REG_CR is what turns the receiver on.
const MAC_TRX_ENABLE: u8 = 0xFF;
// Latch the loaded reserved-queue page numbers.
const LD_RQPN: u32 = 1 << 31;
// Beacon-queue full marker for the 11ac H2C path.
const H2CQ_FULL: u32 = 1 << 31;
// BIT_EN_WR_FREE_TAIL (BIT20) seen in the third byte of REG_FWHW_TXQ_CTRL.
const EN_WR_FREE_TAIL_B2: u8 = 0x10;
// Trigger and status of the automatic link-list-table init.
const AUTO_INIT_LLT_V1: u8 = 0x01;
// BIT_DROP_DATA_EN (BIT15) in the high byte of REG_TXDMA_OFFSET_CHK.
const DROP_DATA_EN_B1: u8 = 0x80;

// FIFO geometry for the 8821c (rtw8821c.c chip params, main.h page size).
const TX_PAGE_SIZE_SHIFT: u32 = 7;
const TXFF_SIZE: u32 = 65536;
const RXFF_SIZE: u32 = 16384;
const C2H_PKT_BUF: u32 = 256;
// Reserved driver/firmware pages that sit above the addressable queues.
const RSVD_DRV_PG_NUM: u32 = 8;
const H2C_EXTRAINFO: u32 = 24;
const H2C_STATICINFO: u32 = 8;
const RSVD_H2CQ_NUM: u32 = 8;
const CPU_INSTRUCTION: u32 = 0;
const FW_TXBUF: u32 = 4;
const CSI_BUF: u32 = 0;

// page_table_8821c[1] (PCIe): the per-priority queue page counts.
const HQ_NUM: u16 = 16;
const NQ_NUM: u16 = 16;
const LQ_NUM: u16 = 16;
const EXQ_NUM: u16 = 14;
const GAPQ_NUM: u16 = 1;

// Total transmit FIFO pages, and how many are left for the addressable queues
// once the reserved pages are taken out.
const TXFF_PG_NUM: u32 = TXFF_SIZE >> TX_PAGE_SIZE_SHIFT;
const RSVD_PG_NUM: u32 = RSVD_DRV_PG_NUM
    + H2C_EXTRAINFO
    + H2C_STATICINFO
    + RSVD_H2CQ_NUM
    + CPU_INSTRUCTION
    + FW_TXBUF
    + CSI_BUF;
const ACQ_PG_NUM: u32 = TXFF_PG_NUM - RSVD_PG_NUM;
// The reserved-page boundary the beacon and public queues are bounded against.
const RSVD_BOUNDARY: u16 = (TXFF_PG_NUM - RSVD_PG_NUM) as u16;
// The public queue takes whatever pages the per-priority queues do not.
const PUBQ_NUM: u16 = ACQ_PG_NUM as u16 - HQ_NUM - LQ_NUM - NQ_NUM - EXQ_NUM - GAPQ_NUM;
// The receive FIFO boundary leaves room for the C2H report buffer.
const RXFF_BNDY: u32 = RXFF_SIZE - C2H_PKT_BUF - 1;

// The H2C queue lives just below the firmware TX buffer, walking down from the
// top of the FIFO exactly as rtw88 sites it.
const CSIBUF_ADDR: u32 = TXFF_PG_NUM - CSI_BUF;
const FW_TXBUF_ADDR: u32 = CSIBUF_ADDR - FW_TXBUF;
const CPU_INSTR_ADDR: u32 = FW_TXBUF_ADDR - CPU_INSTRUCTION;
const H2CQ_PAGE: u32 = CPU_INSTR_ADDR - RSVD_H2CQ_NUM;
const H2CQ_ADDR: u32 = H2CQ_PAGE << TX_PAGE_SIZE_SHIFT;
const H2CQ_SIZE: u32 = RSVD_H2CQ_NUM << TX_PAGE_SIZE_SHIFT;
const H2CQ_TAIL: u32 = H2CQ_ADDR + H2CQ_SIZE;
// The H2C address fields keep their top bits; only the low 18 hold the address.
const H2C_ADDR_KEEP: u32 = 0xFFFC_0000;

// rqpn_table_8821c[1] (PCIe): the priority-to-DMA-queue mapping, packed into
// REG_TXDMA_PQ_MAP two bits per queue. vo/vi=NORMAL(2), be/bk=LOW(1),
// mg=EXTRA(0), hi=HIGH(3).
const fn txdma_pq_map() -> u16 {
    const VO: u16 = 2;
    const VI: u16 = 2;
    const BE: u16 = 1;
    const BK: u16 = 1;
    const MG: u16 = 0;
    const HI: u16 = 3;
    (VO << 4) | (VI << 6) | (BE << 8) | (BK << 10) | (MG << 12) | (HI << 14)
}

/// Reads allowed before the link-list-table auto-init is declared stuck.
const LLT_POLL_LIMIT: u32 = 1_000_000;

/// Configure and enable the MAC's transmit and receive engines. Returns false if
/// the link-list table never finished initialising or the H2C queue did not come
/// up with its whole ring free, either of which means the MAC is not usable.
pub fn init_trx_cfg<M: Mmio>(mmio: &M) -> bool {
    queue_mapping(mmio);
    if !priority_queue_cfg(mmio) {
        return false;
    }
    init_h2c(mmio)
}

// txdma_queue_mapping: map the priorities onto the DMA queues, then bracket the
// map write with clearing and setting REG_CR so all eight engine bits come up.
fn queue_mapping<M: Mmio>(mmio: &M) {
    mmio.write16(REG_TXDMA_PQ_MAP, txdma_pq_map());
    mmio.write8(REG_CR, 0);
    mmio.write8(REG_CR, MAC_TRX_ENABLE);
    mmio.write32(REG_H2CQ_CSR, H2CQ_FULL);
}

// priority_queue_cfg: load the per-queue page counts, latch them, set the
// reserved-page boundaries and the receive FIFO boundary, then run the automatic
// link-list-table init and wait for it to finish.
fn priority_queue_cfg<M: Mmio>(mmio: &M) -> bool {
    mmio.write16(REG_FIFOPAGE_INFO_1, HQ_NUM);
    mmio.write16(REG_FIFOPAGE_INFO_2, LQ_NUM);
    mmio.write16(REG_FIFOPAGE_INFO_3, NQ_NUM);
    mmio.write16(REG_FIFOPAGE_INFO_4, EXQ_NUM);
    mmio.write16(REG_FIFOPAGE_INFO_5, PUBQ_NUM);
    set32(mmio, REG_RQPN_CTRL_2, LD_RQPN);
    mmio.write16(REG_FIFOPAGE_CTRL_2, RSVD_BOUNDARY);
    set8(mmio, REG_FWHW_TXQ_CTRL + 2, EN_WR_FREE_TAIL_B2);
    mmio.write16(REG_BCNQ_BDNY_V1, RSVD_BOUNDARY);
    mmio.write16(REG_FIFOPAGE_CTRL_2 + 2, RSVD_BOUNDARY);
    mmio.write16(REG_BCNQ1_BDNY_V1, RSVD_BOUNDARY);
    mmio.write32(REG_RXFF_BNDY, RXFF_BNDY);
    set8(mmio, REG_AUTO_LLT_V1, AUTO_INIT_LLT_V1);
    if !poll_llt(mmio) {
        return false;
    }
    mmio.write8(REG_CR + 3, 0);
    true
}

// init_h2c: point the H2C queue head, read and tail registers at the reserved
// H2C pages, mark its info register, enable the drop-data path, then confirm the
// whole ring reads back free.
fn init_h2c<M: Mmio>(mmio: &M) -> bool {
    rmw32(mmio, REG_H2C_HEAD, H2C_ADDR_KEEP, H2CQ_ADDR);
    rmw32(mmio, REG_H2C_READ_ADDR, H2C_ADDR_KEEP, H2CQ_ADDR);
    rmw32(mmio, REG_H2C_TAIL, H2C_ADDR_KEEP, H2CQ_TAIL);
    rmw8(mmio, REG_H2C_INFO, 0xFC, 0x01);
    rmw8(mmio, REG_H2C_INFO, 0xFB, 0x04);
    rmw8(mmio, REG_TXDMA_OFFSET_CHK + 1, 0x7F, DROP_DATA_EN_B1);

    let wp = mmio.read32(REG_H2C_PKT_WRITEADDR) & 0x3FFFF;
    let rp = mmio.read32(REG_H2C_PKT_READADDR) & 0x3FFFF;
    let free = if wp >= rp { H2CQ_SIZE - (wp - rp) } else { rp - wp };
    free == H2CQ_SIZE
}

// Wait for the auto-init link-list-table bit to clear.
fn poll_llt<M: Mmio>(mmio: &M) -> bool {
    for _ in 0..LLT_POLL_LIMIT {
        if mmio.read8(REG_AUTO_LLT_V1) & AUTO_INIT_LLT_V1 == 0 {
            return true;
        }
        core::hint::spin_loop();
    }
    false
}

/// Reset the transmit and receive ring pointers and the DMA interface once the
/// ring addresses are programmed. This is rtw88 `rtw_pci_reset_buf_desc` (its
/// pointer-clear tail) plus `rtw_pci_dma_reset`: clear the ring read/write
/// pointers, clear the H2C queue host and hardware indices, then reset the TRX
/// DMA interface with the receive tag enabled. Without the interface reset the
/// card never advances its receive write index, so no frame is ever delivered.
/// Call after the transmit and receive rings have their addresses and lengths
/// set.
pub fn reset_trx_dma<M: Mmio>(mmio: &M) {
    mmio.write32(REG_TXBD_RWPTR_CLR, 0xFFFF_FFFF);
    let csr = mmio.read32(REG_H2CQ_CSR);
    mmio.write32(REG_H2CQ_CSR, csr | CLR_H2CQ_HOST_IDX | CLR_H2CQ_HW_IDX);
    let ctrl = mmio.read32(REG_PCI_CTRL);
    mmio.write32(REG_PCI_CTRL, ctrl | RST_TRXDMA_INTF | RX_TAG_EN);
}

fn set8<M: Mmio>(mmio: &M, off: usize, bit: u8) {
    let v = mmio.read8(off);
    mmio.write8(off, v | bit);
}

fn set32<M: Mmio>(mmio: &M, off: usize, bit: u32) {
    let v = mmio.read32(off);
    mmio.write32(off, v | bit);
}

// Replace only the bits outside `keep` with `set`, leaving the kept bits as the
// hardware brought them up.
fn rmw8<M: Mmio>(mmio: &M, off: usize, keep: u8, set: u8) {
    let v = mmio.read8(off);
    mmio.write8(off, (v & keep) | set);
}

fn rmw32<M: Mmio>(mmio: &M, off: usize, keep: u32, set: u32) {
    let v = mmio.read32(off);
    mmio.write32(off, (v & keep) | set);
}
