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

//! The PCI transfer-ring registers for the best-effort (BE) TX queue, which
//! carries ordinary data frames. Each queue has three registers: the 64-bit bus
//! address of its buffer-descriptor ring (`DESA`), the ring length (`NUM`), and
//! the host/hardware read-write index (`IDX`). Values are the rtw88 `pci.h`
//! `RTK_PCI_TXBD_*_BEQ` addresses. The index register packs the host write index
//! in its low 12 bits and the hardware read index in bits 16..27.

/// `RTK_PCI_TXBD_DESA_BEQ`: bus address of the BE-queue buffer-descriptor ring.
pub const REG_TXBD_DESA_BEQ: usize = 0x0328;
/// `RTK_PCI_TXBD_NUM_BEQ`: the BE-queue ring length, in descriptors.
pub const REG_TXBD_NUM_BEQ: usize = 0x0388;
/// `RTK_PCI_TXBD_IDX_BEQ`: the BE-queue read-write index. Writing the low 12
/// bits sets the host write index and kicks the queue; the hardware read index
/// reads back in bits 16..27.
pub const REG_TXBD_IDX_BEQ: usize = 0x03A8;

// The transfer-ring index registers pack a host index in the low 12 bits and a
// hardware index in bits 16..27; the RX queue shares this layout and imports
// these (`TRX_BD_IDX_MASK` / `TRX_BD_HW_IDX_MASK` in rtw88 `pci.h`).
/// `TRX_BD_IDX_MASK` (`GENMASK(11, 0)`): the host index field.
pub const TRX_BD_IDX_MASK: u32 = 0x0FFF;
/// `TRX_BD_HW_IDX_MASK` (`GENMASK(27, 16)`): the hardware index field.
pub const TRX_BD_HW_IDX_MASK: u32 = 0x0FFF_0000;
/// The shift of the hardware index within the index register.
pub const TRX_BD_HW_IDX_SHIFT: u32 = 16;

/// The queue selector for best-effort data is TID 0.
pub const QSEL_BE: u8 = 0;
