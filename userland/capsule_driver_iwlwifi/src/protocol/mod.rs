// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

mod decode;
mod encode;
mod errno;
mod header;
mod limits;
mod ops;

pub use decode::parse;
pub use encode::response;
pub use errno::{E_BAD_OP, E_FW_INVALID, E_INVAL, E_OK, E_TIMEOUT};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{FW_NAME_MAX, IPC_PAYLOAD_MAX};
pub use ops::{
    OP_ALIVE_WAIT, OP_BEACON_PARSE, OP_DEVICE_INFO, OP_DMA_STATE, OP_FIRMWARE_INFO,
    OP_FIRMWARE_LOAD, OP_FIRMWARE_STAGE, OP_HCMD_ISSUE, OP_HEALTHCHECK, OP_MGMT_BUILD, OP_RF_STATE,
    OP_EAPOL_VERIFY, OP_RX_POLL, OP_WPA_PTK,
};
