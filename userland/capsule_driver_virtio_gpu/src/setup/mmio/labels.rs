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
pub fn errno_label(errno: i64) -> &'static str {
    match errno {
        -1 => "virtio-gpu: mmio map denied",
        -12 => "virtio-gpu: mmio map no memory",
        -19 => "virtio-gpu: mmio map no device",
        -22 => "virtio-gpu: mmio map invalid range",
        -95 => "virtio-gpu: mmio map unsupported flags",
        -116 => "virtio-gpu: mmio map stale claim",
        _ => "virtio-gpu: mmio map failed",
    }
}
pub fn pio_errno_label(errno: i64) -> &'static str {
    match errno {
        -1 => "virtio-gpu: pio grant denied",
        -12 => "virtio-gpu: pio grant no memory",
        -19 => "virtio-gpu: pio grant no device",
        -22 => "virtio-gpu: pio grant invalid range",
        -95 => "virtio-gpu: pio grant unsupported flags",
        -116 => "virtio-gpu: pio grant stale claim",
        _ => "virtio-gpu: pio grant failed",
    }
}
