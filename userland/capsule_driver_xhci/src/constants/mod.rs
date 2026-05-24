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
mod cap_offsets;
mod completion_codes;
mod erdp_bits;
mod interrupter_bits;
mod op_offsets;
mod pci_class;
mod portsc_bits;
mod ring;
mod runtime_offsets;
mod trb_flags;
mod trb_kinds;
mod usbcmd_bits;
mod usbsts_bits;
pub use cap_offsets::{CAPLENGTH, DBOFF, HCCPARAMS1, HCSPARAMS1, HCSPARAMS2, RTSOFF};
pub use completion_codes::{CC_SHORT_PACKET, CC_SUCCESS};
pub use erdp_bits::ERDP_EHB;
pub use interrupter_bits::{IMAN_IE, IMAN_IP};
pub use op_offsets::{CONFIG, CRCR_LO, DCBAAP_LO, PORTSC_BASE, PORT_REG_STRIDE, USBCMD, USBSTS};
pub use pci_class::CLASS_USB_HOST_XHCI;
pub use portsc_bits::{
    PORTSC_CCS, PORTSC_CHANGE_BITS, PORTSC_PED, PORTSC_PLS_MASK, PORTSC_PR, PORTSC_PRC,
    PORTSC_SPEED_MASK, PORTSC_SPEED_SHIFT,
};
pub use ring::{
    COMMAND_RING_TRBS, EVENT_RING_SEGMENT_TABLE_ENTRIES, EVENT_RING_SEGMENT_TRBS, TRB_BYTES,
};
pub use runtime_offsets::{ERDP_LO, ERSTBA_LO, ERSTSZ, IMAN, IMOD, INTERRUPTER_STRIDE};
pub use trb_flags::{
    LINK_TC, TRB_CH, TRB_CYCLE, TRB_DIR_IN, TRB_IDT, TRB_IOC, TRB_ISP, TRB_TYPE_MASK,
    TRB_TYPE_SHIFT, TRT_IN_DATA, TRT_NO_DATA, TRT_OUT_DATA,
};
pub use trb_kinds::{
    TRB_TYPE_ADDRESS_DEVICE_CMD, TRB_TYPE_CMD_COMPLETION_EVENT, TRB_TYPE_DATA_STAGE,
    TRB_TYPE_DISABLE_SLOT_CMD, TRB_TYPE_ENABLE_SLOT_CMD, TRB_TYPE_LINK, TRB_TYPE_NOOP_CMD,
    TRB_TYPE_NORMAL, TRB_TYPE_SETUP_STAGE, TRB_TYPE_STATUS_STAGE, TRB_TYPE_TRANSFER_EVENT,
};
pub use usbcmd_bits::{USBCMD_HCRST, USBCMD_INTE, USBCMD_RUN};
pub use usbsts_bits::{USBSTS_CNR, USBSTS_HCH, USBSTS_HSE};