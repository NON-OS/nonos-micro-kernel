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
mod notify;
mod ops;

pub use decode::parse;
pub use encode::{response_header, write_status};
pub use errno::{E_BAD_OP, E_INVAL, E_NOENT, E_NOMEM, E_PERM};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{
    IPC_PAYLOAD_MAX, QUERY_FOCUS_RESP_LEN, QUERY_TOPMOST_REQ_LEN, QUERY_TOPMOST_RESP_LEN,
    ROUTE_FOCUS_REQ_LEN, STATUS_LEN, WINDOW_CLOSE_REQ_LEN, WINDOW_FOCUS_REQ_LEN,
    WINDOW_MINIMIZE_REQ_LEN, WINDOW_MOVE_REQ_LEN, WINDOW_OPEN_REQ_LEN, WINDOW_RAISE_REQ_LEN,
    WINDOW_RESIZE_REQ_LEN, WINDOW_RESTORE_REQ_LEN,
};
pub use notify::{encode_notify, NOTIFY_KIND_CLOSED, NOTIFY_KIND_OPENED, NOTIFY_LEN};
pub use ops::{
    OP_HEALTHCHECK, OP_LIFECYCLE_SUBSCRIBE, OP_QUERY_FOCUS, OP_QUERY_TOPMOST, OP_ROUTE_FOCUS,
    OP_WINDOW_CLOSE, OP_WINDOW_FOCUS, OP_WINDOW_MINIMIZE, OP_WINDOW_MOVE, OP_WINDOW_OPEN,
    OP_WINDOW_RAISE, OP_WINDOW_RESIZE, OP_WINDOW_RESTORE,
};
