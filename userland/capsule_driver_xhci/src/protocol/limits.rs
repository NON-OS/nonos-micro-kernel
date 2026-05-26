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
pub const CONTROLLER_STATUS_PAYLOAD_LEN: usize = 56;
pub const PORT_ENTRY_BYTES: usize = 8;
pub const MAX_PORTS_REPORTED: usize = 255;
pub const PORT_STATUS_HEADER_BYTES: usize = 4;
pub const MAX_REQUEST_PAYLOAD_LEN: usize = 10;
pub const SLOT_ENABLE_PAYLOAD_LEN: usize = 4;
pub const SLOT_DISABLE_PAYLOAD_LEN: usize = 1;
pub const ADDRESS_DEVICE_REQUEST_LEN: usize = 2;
pub const ADDRESS_DEVICE_REPLY_LEN: usize = 8;
pub const DEVICE_DESCRIPTOR_REQUEST_LEN: usize = 1;
pub const DEVICE_DESCRIPTOR_REPLY_LEN: usize = 18;
pub const CONFIG_DESCRIPTOR_REQUEST_LEN: usize = 4;
pub const CONFIG_DESCRIPTOR_REPLY_PREFIX: usize = 4;
pub const CONTROL_TRANSFER_REQUEST_LEN: usize = 10;
pub const CONTROL_TRANSFER_REPLY_PREFIX: usize = 2;
pub const ALLOC_TRANSFER_RING_REQUEST_LEN: usize = 6;
pub const ALLOC_TRANSFER_RING_REPLY_LEN: usize = 4;
pub const INTERRUPT_IN_REQUEST_LEN: usize = 4;
pub const INTERRUPT_IN_REPLY_PREFIX: usize = 2;
pub const HID_REPORT_MAX: usize = 8;