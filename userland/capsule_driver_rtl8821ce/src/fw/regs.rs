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

//! The MAC registers and bits the firmware download touches, beyond the DDMA
//! channel in `ddma`. These are the stable rtw88 MAC map (`reg.h`), the values
//! reimplemented here with the rtw88 name of each in a comment so the program is
//! traceable to its origin. They cover: halting and releasing the on-chip 8051,
//! the reserved-page (beacon-queue) staging path firmware chunks travel through,
//! and the firmware-ready handshake that ends the download.

// On-chip 8051 control.
/// `REG_SYS_FUNC_EN`, byte 1 carries `BIT_FEN_CPUEN` (the 8051 run enable).
pub const REG_SYS_FUNC_EN: usize = 0x0002;
/// `BIT_FEN_CPUEN` sits in bit 2 of the high byte of `REG_SYS_FUNC_EN`.
pub const FEN_CPUEN_HI: u8 = 1 << 2;
/// `REG_RSV_CTRL`, byte 1 carries `BIT_WLMCU_IOIF` (the 8051 IO interface).
pub const REG_RSV_CTRL: usize = 0x001C;
/// `BIT_WLMCU_IOIF` is bit 0 of the high byte of `REG_RSV_CTRL`.
pub const WLMCU_IOIF_HI: u8 = 1 << 0;

/// `REG_CPU_DMEM_CON`; byte 2 carries `BIT_WL_PLATFORM_RST` (`BIT(16)`).
pub const REG_CPU_DMEM_CON: usize = 0x1080;
/// `BIT_WL_PLATFORM_RST >> 16`, the platform-reset bit in byte 2.
pub const WL_PLATFORM_RST_B2: u8 = 1 << 0;
/// `REG_SYS_CLK_CTRL`; byte 1 carries `BIT_CPU_CLK_EN` (`BIT(14)`).
pub const REG_SYS_CLK_CTRL: usize = 0x0008;
/// `BIT_CPU_CLK_EN >> 8`, the 8051 clock-enable bit in byte 1.
pub const CPU_CLK_EN_HI: u8 = 1 << 6;

// Firmware-download control and the ready handshake (`REG_MCUFW_CTRL`, 0x0080).
/// `REG_MCUFW_CTRL`: the firmware-download enable and ready-state register.
pub const REG_MCUFW_CTRL: usize = 0x0080;
/// `BIT_MCUFWDL_EN`, enables the download path.
pub const MCUFWDL_EN: u16 = 1 << 0;
/// `BIT_IMEM_DW_OK`: the IMEM section downloaded.
pub const IMEM_DW_OK: u16 = 1 << 3;
/// `BIT_IMEM_CHKSUM_OK`: the IMEM checksum validated.
pub const IMEM_CHKSUM_OK: u16 = 1 << 4;
/// `BIT_DMEM_DW_OK`: the DMEM section downloaded.
pub const DMEM_DW_OK: u16 = 1 << 5;
/// `BIT_DMEM_CHKSUM_OK`: the DMEM checksum validated.
pub const DMEM_CHKSUM_OK: u16 = 1 << 6;
/// `BIT_FW_DW_RDY`: the firmware finished loading.
pub const FW_DW_RDY: u16 = 1 << 14;
/// `BIT_FW_INIT_RDY`: the firmware finished initialising on the 8051.
pub const FW_INIT_RDY: u16 = 1 << 15;
/// `BIT_CHECK_SUM_OK = BIT(4) | BIT(6)`, both section checksums good.
pub const CHECK_SUM_OK: u16 = IMEM_CHKSUM_OK | DMEM_CHKSUM_OK;

// The reserved-page staging path. Firmware chunks are written into the on-chip
// packet buffer through the beacon queue, one page-0 write per chunk.
/// `REG_CR` (MAC control). Byte 1 carries `BIT_ENSWBCN` (`BIT(8)`).
pub const REG_CR: usize = 0x0100;
/// `BIT_ENSWBCN >> 8`, software-beacon enable in byte 1 of `REG_CR`.
pub const ENSWBCN_HI: u8 = 1 << 0;
/// `REG_FIFOPAGE_CTRL_2`; the reserved-page head and the beacon-valid status.
pub const REG_FIFOPAGE_CTRL_2: usize = 0x0204;
/// `BIT_BCN_VALID_V1` (`BIT(15)`): written 1 to clear, set by hardware when the
/// reserved-page write has landed. The reserved-page head (the low 12 bits of
/// this register) stays 0 throughout firmware download.
pub const BCN_VALID_V1: u16 = 1 << 15;
/// `REG_FWHW_TXQ_CTRL`; byte 2 carries `BIT_EN_BCNQ_DL` (`BIT(22)`).
pub const REG_FWHW_TXQ_CTRL: usize = 0x0420;
/// `BIT_EN_BCNQ_DL >> 16`, beacon-queue download enable in byte 2.
pub const EN_BCNQ_DL_B2: u8 = 1 << 6;

// The beacon-queue PCI transfer ring, used only to stage reserved pages here.
/// `RTK_PCI_TXBD_DESA_BCNQ`: the 64-bit bus address of the beacon-queue ring.
pub const REG_TXBD_DESA_BCNQ: usize = 0x0308;
/// `RTK_PCI_TXBD_BCN_WORK`: writing `BIT_PCI_BCNQ_FLAG` kicks a beacon transfer.
pub const REG_TXBD_BCN_WORK: usize = 0x0383;
/// `BIT_PCI_BCNQ_FLAG` (`BIT(4)`), the beacon-queue kick.
pub const PCI_BCNQ_FLAG: u8 = 1 << 4;

// The on-chip packet buffer the DDMA copies firmware out of. rtw88 stages every
// chunk at reserved-page 0, so the DDMA source is a constant.
/// `OCPBASE_TXBUF_88XX`: base of the on-chip TX packet buffer.
pub const OCPBASE_TXBUF: u32 = 0x1878_0000;

/// `OCPBASE_DMEM_88XX`: a section whose on-chip destination is at or above this
/// lands in DMEM, below it in IMEM. rtw88 uses this split to decide which pair of
/// download-ok/checksum-ok bits to set after a section validates.
pub const OCPBASE_DMEM: u32 = 0x0020_0000;

// The TX packet and buffer descriptor sizes are Realtek PCI ring facts shared
// with the data TX path; they live in `ring` and are re-exported here so the
// firmware download and its proofs keep one import site.
pub use crate::ring::{TX_BUF_DESC_SIZE, TX_DESC_SIZE};

/// `TX_DESC_QSEL_BEACON`: the queue selector for a reserved (beacon) page.
pub const QSEL_BEACON: u32 = 16;

// Download prologue and epilogue: the queue mapping the reserved-page path needs,
// the platform reset around a reload, and the firmware-ready handshake. From
// rtw88 `download_firmware_reg_backup`, `download_firmware_reset_platform`,
// `download_firmware_end_flow` and `download_firmware_validate` in `mac.c`.

/// `REG_TXDMA_PQ_MAP`; byte 1 sets the priority-queue mapping for download.
pub const REG_TXDMA_PQ_MAP: usize = 0x010C;
/// `RTW_DMA_MAPPING_HIGH << 6`: map the high queue to high priority in byte 1.
pub const DMA_MAPPING_HIGH_B1: u8 = 3 << 6;
/// `BIT_HCI_TXDMA_EN | BIT_TXDMA_EN`: the TX DMA enables set in `REG_CR` byte 0
/// so the beacon queue can move a staged page.
pub const CR_TXDMA_EN: u8 = (1 << 0) | (1 << 2);
/// `REG_H2CQ_CSR`: the host-to-card queue control, cleared full for download.
pub const REG_H2CQ_CSR: usize = 0x1330;
/// `BIT_H2CQ_FULL`.
pub const H2CQ_FULL: u32 = 1 << 31;
/// `REG_FIFOPAGE_INFO_1`: the high-priority-queue page count, 0x200 for download.
pub const REG_FIFOPAGE_INFO_1: usize = 0x0230;
/// The page count programmed into `REG_FIFOPAGE_INFO_1` during download.
pub const FIFOPAGE_INFO_1_DLFW: u16 = 0x0200;
/// `REG_RQPN_CTRL_2`: report-queue-page-number load control.
pub const REG_RQPN_CTRL_2: usize = 0x022C;
/// `BIT_LD_RQPN`: load the report-queue page numbers.
pub const LD_RQPN: u32 = 1 << 31;
/// `REG_BCN_CTRL`: beacon control; the beacon function is disabled for download.
pub const REG_BCN_CTRL: usize = 0x0550;
/// `BIT_EN_BCN_FUNCTION`: the beacon function enable, cleared for download.
pub const EN_BCN_FUNCTION: u8 = 1 << 3;
/// `BIT_DIS_TSF_UDT`: disable TSF update, set for download.
pub const DIS_TSF_UDT: u8 = 1 << 4;
/// `REG_TXDMA_STATUS`: cleared of the page-overflow flag to end download.
pub const REG_TXDMA_STATUS: usize = 0x0210;
/// `BTI_PAGE_OVF`: the page-overflow status bit.
pub const BTI_PAGE_OVF: u32 = 1 << 2;

/// `FW_READY_MASK`: the low 16 bits of `REG_MCUFW_CTRL` carry the ready state.
pub const FW_READY_MASK: u16 = 0xFFFF;
/// `FW_READY = FW_INIT_RDY | FW_DW_RDY | IMEM_DW_OK | DMEM_DW_OK | CHECK_SUM_OK`:
/// the value that reads back once the firmware has loaded and initialised.
pub const FW_READY: u16 = FW_INIT_RDY | FW_DW_RDY | IMEM_DW_OK | DMEM_DW_OK | CHECK_SUM_OK;
