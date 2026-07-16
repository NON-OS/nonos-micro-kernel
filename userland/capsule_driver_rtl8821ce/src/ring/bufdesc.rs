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

//! Build a Realtek PCI TX buffer descriptor: a pair of 8-byte entries
//! (`rtw_pci_tx_buffer_desc`: `{ u16 buf_size; u16 psb_len; u32 dma }`) that tell
//! the card where a packet lives in host memory. The first entry points at the
//! 48-byte TX packet descriptor and also carries `psb_len`, the packet's size in
//! 128-byte packet-buffer pages; the second points at the payload right after
//! it. The beacon queue additionally sets the ownership bit in `psb_len`; the
//! data queues leave it clear and hand ownership over by advancing the ring
//! write index. Layout and the `psb_len` maths follow rtw88 `rtw_pci_tx_write_data`
//! in `pci.c`.

use super::{TX_BUF_DESC_SIZE, TX_PAGE_SIZE};

/// `RTK_PCI_TXBD_OWN_OFFSET`: the ownership bit in `psb_len`.
const TXBD_OWN: u16 = 1 << 15;

/// Number of 128-byte pages a `total`-byte packet occupies: `(total-1)/128 + 1`.
fn page_span(total: usize) -> u16 {
    (((total - 1) / TX_PAGE_SIZE) + 1) as u16
}

/// Build the 16-byte buffer descriptor for a packet whose `desc_len`-byte TX
/// descriptor starts at bus address `desc_addr` and whose `payload_len`-byte
/// payload follows it. `own` sets the ownership bit (used by the beacon queue).
pub fn pair(
    desc_addr: u64,
    desc_len: usize,
    payload_len: usize,
    own: bool,
) -> [u8; TX_BUF_DESC_SIZE] {
    let total = desc_len + payload_len;
    let mut psb = page_span(total);
    if own {
        psb |= TXBD_OWN;
    }
    let payload_addr = desc_addr + desc_len as u64;

    let mut d = [0u8; TX_BUF_DESC_SIZE];
    // Entry 0: the TX descriptor. buf_size = desc_len, psb_len = pages [| own].
    d[0..2].copy_from_slice(&(desc_len as u16).to_le_bytes());
    d[2..4].copy_from_slice(&psb.to_le_bytes());
    d[4..8].copy_from_slice(&(desc_addr as u32).to_le_bytes());
    // Entry 1: the payload. buf_size = payload_len, psb_len unused.
    d[8..10].copy_from_slice(&(payload_len as u16).to_le_bytes());
    d[12..16].copy_from_slice(&(payload_addr as u32).to_le_bytes());
    d
}
