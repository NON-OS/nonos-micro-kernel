// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Stage one firmware chunk into the on-chip packet buffer as a reserved page.
//! This is the hardware-specific half of firmware download: the DDMA channel can
//! only copy out of the packet buffer, so each chunk first travels through the
//! beacon queue into reserved-page 0. The engine lays the 48-byte TX descriptor
//! and the payload into a DMA buffer, publishes a beacon buffer descriptor, arms
//! the beacon-valid status, kicks the beacon queue and waits for the card to
//! report the page landed, then returns the DDMA source address the caller
//! copies from. The register sequence follows rtw88 `rtw_fw_write_data_rsvd_page`
//! (the 11ac path) and `rtw_pci_write_data_rsvd_page` in `pci.c`. The exact byte
//! layout and register program are checked against a modeled device in
//! `rtl8821ce_proofs`; only the card actually consuming the beacon page is left
//! for on-silicon bring-up.

use super::dma::DmaMem;
use super::regs::{
    BCN_VALID_V1, ENSWBCN_HI, EN_BCNQ_DL_B2, OCPBASE_TXBUF, PCI_BCNQ_FLAG, REG_CR,
    REG_FIFOPAGE_CTRL_2, REG_FWHW_TXQ_CTRL, REG_TXBD_BCN_WORK, TX_BUF_DESC_SIZE, TX_DESC_SIZE,
};
use super::txdesc::{self, TXDESC_LEN};
use crate::regs::Mmio;

/// Reads allowed before the reserved-page write is declared not to have landed.
const BCN_VALID_POLL_LIMIT: u32 = 1_000_000;

/// Stage `chunk` at reserved-page 0 and return the DDMA source address for it, or
/// `None` if the chunk does not fit the staging buffer or the card never reports
/// the page valid. rtw88 always stages at page 0, so the returned source is the
/// constant packet-buffer address just past the prepended TX descriptor.
pub fn stage_chunk<M, B, R>(mmio: &M, stage_buf: &B, ring: &R, chunk: &[u8]) -> Option<u32>
where
    M: Mmio,
    B: DmaMem,
    R: DmaMem,
{
    if chunk.is_empty()
        || chunk.len() + TXDESC_LEN > stage_buf.capacity()
        || TX_BUF_DESC_SIZE > ring.capacity()
    {
        return None;
    }

    // Lay [TX descriptor | payload] into the staging buffer and publish the
    // beacon buffer descriptor that points the card at it.
    let desc = txdesc::beacon(chunk.len(), chunk.first().copied());
    stage_buf.write_bytes(0, &desc);
    stage_buf.write_bytes(TXDESC_LEN, chunk);
    let bd = crate::ring::bufdesc::pair(stage_buf.device_addr(), TXDESC_LEN, chunk.len(), true);
    ring.write_bytes(0, &bd);

    // Arm the beacon-valid status (write-1-to-clear). The reserved-page head is
    // 0 throughout download, so only the valid bit is written.
    mmio.write16(REG_FIFOPAGE_CTRL_2, BCN_VALID_V1);
    // Enable the software beacon path and hold off the beacon-queue auto
    // download while the page is staged; both are restored afterwards.
    let cr1 = mmio.read8(REG_CR + 1);
    mmio.write8(REG_CR + 1, cr1 | ENSWBCN_HI);
    let txq2 = mmio.read8(REG_FWHW_TXQ_CTRL + 2);
    mmio.write8(REG_FWHW_TXQ_CTRL + 2, txq2 & !EN_BCNQ_DL_B2);
    // Kick the beacon queue to fetch the descriptor and land the reserved page.
    let bw = mmio.read8(REG_TXBD_BCN_WORK);
    mmio.write8(REG_TXBD_BCN_WORK, bw | PCI_BCNQ_FLAG);

    let landed = wait_bcn_valid(mmio);

    // Restore the beacon path whether or not the page landed. During download
    // the reserved-page head stays at 0.
    mmio.write16(REG_FIFOPAGE_CTRL_2, BCN_VALID_V1);
    mmio.write8(REG_FWHW_TXQ_CTRL + 2, txq2);
    mmio.write8(REG_CR + 1, cr1);

    if !landed {
        return None;
    }
    Some(OCPBASE_TXBUF + TX_DESC_SIZE)
}

// The card sets beacon-valid once the staged page has landed in the packet
// buffer; a page that never validates means a wedged or unpowered card.
fn wait_bcn_valid<M: Mmio>(mmio: &M) -> bool {
    for _ in 0..BCN_VALID_POLL_LIMIT {
        if mmio.read16(REG_FIFOPAGE_CTRL_2) & BCN_VALID_V1 != 0 {
            return true;
        }
        core::hint::spin_loop();
    }
    false
}
