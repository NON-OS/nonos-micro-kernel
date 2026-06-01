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
pub const SCENE_SUBMIT_REQ_LEN: usize = 32;
pub const DAMAGE_COMMIT_REQ_LEN: usize = 16;
pub const FOCUS_SET_REQ_LEN: usize = 8;
pub const CURSOR_UPDATE_REQ_LEN: usize = 16;
pub const SCENE_REMOVE_REQ_LEN: usize = 8;
pub const DISPLAY_INFO_DATA_LEN: usize = 16;
pub const DISPLAY_INFO_RESP_LEN: usize = STATUS_LEN + DISPLAY_INFO_DATA_LEN;
