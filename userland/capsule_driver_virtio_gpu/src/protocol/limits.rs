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
pub const CONTROLLER_INFO_LEN: usize = 40;
pub const DISPLAY_INFO_LEN: usize = 12;
pub const CONTROLQ_STATE_LEN: usize = 24;
pub const CREATE_RESOURCE_REQ_LEN: usize = 16;
pub const ATTACH_BACKING_REQ_LEN: usize = 24;
pub const TRANSFER_TO_HOST_REQ_LEN: usize = 32;
pub const SET_SCANOUT_REQ_LEN: usize = 24;
pub const FLUSH_REQ_LEN: usize = 20;
pub const QUERY_CAPS_RESP_LEN: usize = 12;
pub const MODE_LIST_ENTRY_LEN: usize = 32;
pub const GET_PRIMARY_SURFACE_RESP_LEN: usize = 32;
pub const MAX_RESOURCES: usize = 64;