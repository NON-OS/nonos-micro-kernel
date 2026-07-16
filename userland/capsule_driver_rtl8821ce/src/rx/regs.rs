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

//! The PCI transfer-ring registers for the MPDU RX queue and the interrupt
//! mask/status. Received frames land in a ring whose buffer-descriptor base is
//! `DESA`, whose length is `NUM`, and whose host/hardware index is `IDX` (the
//! hardware write index in bits 16..27, the host read index in the low 12 bits).
//! `HIMR0`/`HISR0` mask and report the RX-OK and RX-descriptor-unavailable
//! interrupts. Values are the rtw88 `pci.h` `RTK_PCI_RXBD_*_MPDUQ` and interrupt
//! definitions.

/// `RTK_PCI_RXBD_DESA_MPDUQ`: bus address of the RX buffer-descriptor ring.
pub const REG_RXBD_DESA_MPDUQ: usize = 0x0338;
/// `RTK_PCI_RXBD_NUM_MPDUQ`: the RX ring length, in descriptors.
pub const REG_RXBD_NUM_MPDUQ: usize = 0x0382;
/// `RTK_PCI_RXBD_IDX_MPDUQ`: the RX read-write index. The hardware write index
/// reads back in bits 16..27; writing the low 12 bits advances the host read
/// index to hand the descriptors back.
pub const REG_RXBD_IDX_MPDUQ: usize = 0x03B4;

/// `chip->rx_pkt_desc_sz`: the RX descriptor prefixed to each received frame.
pub const RX_PKT_DESC_SIZE: usize = 24;
/// `chip->rx_buf_desc_sz`: one RX buffer descriptor.
pub const RX_BUF_DESC_SIZE: usize = 8;
