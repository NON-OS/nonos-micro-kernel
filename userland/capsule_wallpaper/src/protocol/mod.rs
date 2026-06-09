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
mod read_u16;
mod read_u32;

pub use decode::parse;
pub use encode::{response_header, write_status};
pub use errno::{E_ACCES, E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_VERSION, E_INVAL};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{
    FADE_REQ_LEN, GET_WALLPAPER_RESP_LEN, IPC_PAYLOAD_MAX, SET_POLICY_REQ_LEN,
    SET_WALLPAPER_REQ_LEN, STATUS_LEN,
};
pub use ops::{OP_FADE, OP_GET_WALLPAPER, OP_HEALTHCHECK, OP_SET_POLICY, OP_SET_WALLPAPER};
pub use read_u16::read_u16;
pub use read_u32::read_u32;
