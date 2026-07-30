// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

pub const TPM_MMIO_BASE: u64 = 0xFED4_0000;
pub const TPM_MMIO_SIZE: usize = 0x5000;

pub const TPM_ACCESS: u32 = 0x0000;
pub const TPM_STS: u32 = 0x0018;
pub const TPM_DATA_FIFO: u32 = 0x0024;
pub const TPM_INTERFACE_ID: u32 = 0x0030;
pub const TPM_DID_VID: u32 = 0x0F00;

/// Upper half of the 64-bit `TPM_CRB_INTF_ID` at 0x30: vendor id in the low
/// sixteen bits, device id in the high sixteen.
///
/// A CRB part carries its identity here and not at 0x0F00, which belongs to
/// the FIFO register file and reads back as zero on CRB. Identity therefore
/// has to come from whichever file the interface actually presents.
pub const TPM_CRB_VID_DID: u32 = 0x0034;

/// `InterfaceType`, the low nibble of `TPM_CRB_INTF_ID`.
pub const TPM_INTF_TYPE_MASK: u32 = 0x0F;

/// `InterfaceType` for a FIFO (TIS) register file.
pub const TPM_INTF_TYPE_FIFO: u32 = 0x0;

/// `InterfaceType` for a Command Response Buffer register file. Every firmware
/// TPM presents this one: Intel PTT and AMD fTPM both do, as does QEMU's
/// `tpm-crb`.
pub const TPM_INTF_TYPE_CRB: u32 = 0x1;

/// `TPM_LOC_CTRL`, where a CRB part takes locality requests. The FIFO file
/// asks for locality by writing bits of `TPM_ACCESS` at 0x00 instead; on CRB
/// that offset is `TPM_LOC_STATE` and read-only, so a FIFO-style request
/// writes into a register that cannot grant anything.
pub const TPM_LOC_CTRL: u32 = 0x0008;

/// `TPM_LOC_STS`, which reports whether the request above was granted.
pub const TPM_LOC_STS: u32 = 0x000C;

/// `requestAccess` in `TPM_LOC_CTRL`.
pub const TPM_LOC_CTRL_REQUEST: u8 = 0x01;

/// `relinquish` in `TPM_LOC_CTRL`.
pub const TPM_LOC_CTRL_RELINQUISH: u8 = 0x02;

/// `granted` in `TPM_LOC_STS`.
pub const TPM_LOC_STS_GRANTED: u8 = 0x01;

// CRB command transport. A CRB part does not take command bytes through a
// FIFO: it publishes the physical address and size of a command buffer and a
// response buffer, and the driver writes the command into memory and rings a
// doorbell. These are the registers that describe those buffers.

/// `TPM_CRB_CTRL_REQ`: ask the part to become ready, or to go idle.
pub const TPM_CRB_CTRL_REQ: u32 = 0x0040;

/// `TPM_CRB_CTRL_STS`: readiness and error state.
pub const TPM_CRB_CTRL_STS: u32 = 0x0044;

/// `TPM_CRB_CTRL_START`: the doorbell. Writing 1 runs the command in the
/// buffer; the part clears it when the response is ready.
pub const TPM_CRB_CTRL_START: u32 = 0x004C;

/// `TPM_CRB_CTRL_CMD_SIZE`: how large the command buffer is.
pub const TPM_CRB_CTRL_CMD_SIZE: u32 = 0x0058;

/// `TPM_CRB_CTRL_CMD_LADDR`: low half of the command buffer's address.
pub const TPM_CRB_CTRL_CMD_LADDR: u32 = 0x005C;

/// `TPM_CRB_CTRL_CMD_HADDR`: high half of the command buffer's address.
pub const TPM_CRB_CTRL_CMD_HADDR: u32 = 0x0060;

/// `TPM_CRB_CTRL_RSP_SIZE`: how large the response buffer is.
pub const TPM_CRB_CTRL_RSP_SIZE: u32 = 0x0064;

/// `TPM_CRB_CTRL_RSP_LADDR`: low half of the response buffer's address.
pub const TPM_CRB_CTRL_RSP_LADDR: u32 = 0x0068;

/// `TPM_CRB_CTRL_RSP_HADDR`: high half of the response buffer's address.
pub const TPM_CRB_CTRL_RSP_HADDR: u32 = 0x006C;

/// `cmdReady` in `TPM_CRB_CTRL_REQ`.
pub const TPM_CRB_REQ_COMMAND_READY: u32 = 0x01;

/// `goIdle` in `TPM_CRB_CTRL_REQ`.
pub const TPM_CRB_REQ_GO_IDLE: u32 = 0x02;

/// `tpmIdle` in `TPM_CRB_CTRL_STS`. Set while the part is not ready for a
/// command.
pub const TPM_CRB_STS_TPM_IDLE: u32 = 0x02;

/// `start` in `TPM_CRB_CTRL_START`.
pub const TPM_CRB_START_GO: u32 = 0x01;

pub const TPM_ACCESS_VALID: u8 = 0x80;
pub const TPM_ACCESS_ACTIVE: u8 = 0x20;
pub const TPM_ACCESS_REQUEST: u8 = 0x02;

pub const TPM_STS_VALID: u8 = 0x80;
pub const TPM_STS_READY: u8 = 0x40;
pub const TPM_STS_GO: u8 = 0x20;
pub const TPM_STS_DATA_AVAIL: u8 = 0x10;
pub const TPM_STS_DATA_EXPECT: u8 = 0x08;
