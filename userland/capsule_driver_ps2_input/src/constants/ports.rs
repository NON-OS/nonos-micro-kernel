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
pub const DATA_OFFSET: u16 = 0;
pub const STATUS_OFFSET: u16 = 4;
pub const CTL_READ_CONFIG: u8 = 0x20;
pub const CTL_WRITE_CONFIG: u8 = 0x60;
pub const CTL_ENABLE_AUX: u8 = 0xA8;
pub const CTL_ENABLE_KBD: u8 = 0xAE;
pub const CTL_WRITE_AUX: u8 = 0xD4;
pub const KBD_ENABLE_SCANNING: u8 = 0xF4;
pub const MOUSE_ENABLE_REPORTING: u8 = 0xF4;
pub const MOUSE_SET_DEFAULTS: u8 = 0xF6;
pub const MOUSE_SET_SAMPLE_RATE: u8 = 0xF3;
pub const MOUSE_GET_DEVICE_ID: u8 = 0xF2;
/// Device id reported after the 200/100/80 sample-rate knock when the mouse has
/// switched to the 4-byte IntelliMouse report with a scroll wheel.
pub const INTELLIMOUSE_ID: u8 = 0x03;
pub const MOUSE_ACK: u8 = 0xFA;
pub const CONFIG_IRQ1: u8 = 1 << 0;
pub const CONFIG_IRQ12: u8 = 1 << 1;
pub const CONFIG_AUX_DISABLE: u8 = 1 << 5;
pub const RING_CAPACITY: usize = 256;
