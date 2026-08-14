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
pub const MOD_DEVICE_FEATURE_SELECT: usize = 0x00;
pub const MOD_DEVICE_FEATURE: usize = 0x04;
pub const MOD_DRIVER_FEATURE_SELECT: usize = 0x08;
pub const MOD_DRIVER_FEATURE: usize = 0x0C;
pub const MOD_DEVICE_STATUS: usize = 0x14;
pub const MOD_QUEUE_SELECT: usize = 0x16;
pub const MOD_QUEUE_SIZE: usize = 0x18;
pub const MOD_QUEUE_ENABLE: usize = 0x1C;
pub const MOD_QUEUE_NOTIFY_OFF: usize = 0x1E;
pub const MOD_QUEUE_DESC: usize = 0x20;
pub const MOD_QUEUE_DRIVER: usize = 0x28;
pub const MOD_QUEUE_DEVICE: usize = 0x30;
pub const FEATURE_PAGE_LOW: u32 = 0;
pub const FEATURE_PAGE_HIGH: u32 = 1;
pub const VIRTIO_F_VERSION_1_HIGH: u32 = 1;
