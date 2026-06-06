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
pub const STATUS_LEN: usize = 4;
pub const MAX_POLL_EVENTS: usize = 256;
pub const EVENT_WIRE_LEN: usize = 3;
pub const MOUSE_EVENT_WIRE_LEN: usize = 8;
pub const STATE_PAYLOAD_LEN: usize = 8 * 7;
pub const CONTROLLER_STATUS_PAYLOAD_LEN: usize = 28;
pub const POLL_PAYLOAD_PREFIX_LEN: usize = STATUS_LEN + 4;
pub const MOUSE_POLL_PAYLOAD_PREFIX_LEN: usize = STATUS_LEN + 4;
