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

pub const IPC_PAYLOAD_MAX: usize = 256;
pub const STATUS_LEN: usize = 4;

// WINDOW_OPEN body: window_id u32, kind u32, x u32, y u32, w u32, h u32
pub const WINDOW_OPEN_REQ_LEN: usize = 24;
// WINDOW_CLOSE body: window_id u32, _pad u32
pub const WINDOW_CLOSE_REQ_LEN: usize = 8;
// WINDOW_MOVE body: window_id u32, _pad u32, x u32, y u32
pub const WINDOW_MOVE_REQ_LEN: usize = 16;
// WINDOW_RESIZE body: window_id u32, _pad u32, w u32, h u32
pub const WINDOW_RESIZE_REQ_LEN: usize = 16;
// WINDOW_FOCUS body: window_id u32, _pad u32
pub const WINDOW_FOCUS_REQ_LEN: usize = 8;
// WINDOW_RAISE body: window_id u32, _pad u32
pub const WINDOW_RAISE_REQ_LEN: usize = 8;
pub const WINDOW_MINIMIZE_REQ_LEN: usize = 8;
pub const WINDOW_RESTORE_REQ_LEN: usize = 8;
pub const QUERY_TOPMOST_REQ_LEN: usize = 8;
pub const QUERY_TOPMOST_RESP_LEN: usize = 16;
pub const ROUTE_FOCUS_REQ_LEN: usize = 8;
