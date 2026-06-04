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
pub const TRB_TYPE_NORMAL: u32 = 1;
pub const TRB_TYPE_SETUP_STAGE: u32 = 2;
pub const TRB_TYPE_DATA_STAGE: u32 = 3;
pub const TRB_TYPE_STATUS_STAGE: u32 = 4;
pub const TRB_TYPE_LINK: u32 = 6;
pub const TRB_TYPE_ENABLE_SLOT_CMD: u32 = 9;
pub const TRB_TYPE_DISABLE_SLOT_CMD: u32 = 10;
pub const TRB_TYPE_ADDRESS_DEVICE_CMD: u32 = 11;
pub const TRB_TYPE_CONFIGURE_ENDPOINT_CMD: u32 = 12;
pub const TRB_TYPE_NOOP_CMD: u32 = 23;
pub const TRB_TYPE_TRANSFER_EVENT: u32 = 32;
pub const TRB_TYPE_CMD_COMPLETION_EVENT: u32 = 33;
