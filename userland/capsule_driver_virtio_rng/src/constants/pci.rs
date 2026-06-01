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

//! PCI identifiers for the virtio-rng device. Both transitional
//! and modern device IDs are recognised; the device the broker
//! reports must match one of them or the driver refuses to drive
//! it.

pub const VIRTIO_VENDOR_ID: u16 = 0x1AF4;
pub const VIRTIO_RNG_TRANSITIONAL: u16 = 0x1005;
pub const VIRTIO_RNG_MODERN: u16 = 0x1044;
