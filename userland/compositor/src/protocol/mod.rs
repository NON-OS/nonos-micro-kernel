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

pub use decode::parse;
pub use encode::{response_header, write_status};
pub use errno::{E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_VERSION, E_INVAL};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{
    CURSOR_UPDATE_REQ_LEN, DAMAGE_COMMIT_REQ_LEN, DISPLAY_INFO_RESP_LEN, FOCUS_SET_REQ_LEN,
    IPC_PAYLOAD_MAX, SCENE_REMOVE_REQ_LEN, SCENE_SUBMIT_REQ_LEN, STATUS_LEN,
};
pub use ops::{
    OP_CURSOR_UPDATE, OP_DAMAGE_COMMIT, OP_DISPLAY_INFO, OP_FOCUS_SET, OP_HEALTHCHECK,
    OP_INPUT_SUBSCRIBE, OP_SCENE_REMOVE, OP_SCENE_SUBMIT,
};
