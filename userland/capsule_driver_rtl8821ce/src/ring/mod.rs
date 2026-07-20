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

//! The Realtek PCI transfer-ring facts shared by the firmware reserved-page
//! staging and the data TX path: the sizes of a TX packet descriptor and a
//! buffer descriptor, the packet-buffer page granularity, and the buffer
//! descriptor builder itself. Keeping them here means one definition drives both
//! paths (`fw::rsvd` stages a beacon page, `tx` sends a data frame).

pub mod bufdesc;

/// `chip->tx_pkt_desc_sz`: the 48-byte packet descriptor prefixed to a frame.
pub const TX_DESC_SIZE: u32 = 48;
/// `chip->tx_buf_desc_sz`: one buffer descriptor (a pair of 8-byte entries).
pub const TX_BUF_DESC_SIZE: usize = 16;
/// `TX_PAGE_SIZE`: the packet-buffer page granularity used for `psb_len`.
pub const TX_PAGE_SIZE: usize = 128;
