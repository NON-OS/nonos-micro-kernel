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

/// Modern (virtio 1.0) PCI register locations, harvested from the device's
/// vendor capabilities (cap_vndr 0x09). Each `cfg_type` names the BAR and the
/// byte offset within it where that structure lives. A capsule cannot read PCI
/// config space, so the kernel parses these and publishes them so the driver
/// can map the correct BAR (not the framebuffer) at the right offset.
#[derive(Clone, Copy, Debug, Default)]
pub struct VirtioPciCfg {
    pub common_bar: u8,
    pub notify_bar: u8,
    pub device_bar: u8,
    pub common_off: u32,
    pub notify_off: u32,
    pub device_off: u32,
    pub isr_off: u32,
    pub notify_mult: u32,
    pub present: bool,
}
