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

use nonos_libc::{DeviceRecord, BUS_KIND_PCI};

use crate::constants::{VIRTIO_GPU_MODERN, VIRTIO_GPU_TRANSITIONAL, VIRTIO_VENDOR_ID};

pub fn is_match(r: &DeviceRecord) -> bool {
    r.vendor == VIRTIO_VENDOR_ID
        && r.bus_kind == BUS_KIND_PCI
        && (r.device == VIRTIO_GPU_TRANSITIONAL || r.device == VIRTIO_GPU_MODERN)
}

pub fn is_usable(r: &DeviceRecord) -> bool {
    is_match(r) && r.irq_pin != 0 && r.irq_line != 0xFF
}
