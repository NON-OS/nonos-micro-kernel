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

mod decode;
mod encode;
mod errno;
mod header;
mod limits;
mod ops;
mod read_i32;
mod read_u16;
mod read_u32;

pub use decode::parse;
pub use encode::{response_header, write_status};
pub use errno::{
    E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_VERSION, E_BUSY, E_INVAL, E_NOENT, E_NOMEM,
};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{
    IPC_PAYLOAD_MAX, NOTIFY_BODY_MAX, NOTIFY_REQ_LEN, STATUS_LEN, TRAY_LABEL_MAX,
    TRAY_REGISTER_REQ_LEN, TRAY_REMOVE_REQ_LEN, TRAY_UPDATE_REQ_LEN,
};
pub use ops::{
    OP_HEALTHCHECK, OP_NOTIFY, OP_SPOTLIGHT_OPEN, OP_TRAY_REGISTER, OP_TRAY_REMOVE, OP_TRAY_UPDATE,
};
pub use read_i32::read_i32;
pub use read_u16::read_u16;
pub use read_u32::read_u32;
