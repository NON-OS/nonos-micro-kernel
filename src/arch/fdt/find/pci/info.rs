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

/// What the device tree says about the PCI host bridge.
///
/// A zero `size` on either window means the bridge did not advertise it, which
/// is normal: plenty of ARM boards expose no PCI I/O space at all.
#[derive(Debug, Clone, Copy, Default)]
pub struct PciHost {
    /// Base of the configuration-space window, addressed by bus/device/function.
    pub ecam_base: u64,
    pub ecam_size: u64,
    /// Where the bridge's I/O window lands in CPU physical address space.
    pub io_cpu_base: u64,
    pub io_size: u64,
    /// The PCI I/O port number `io_cpu_base` corresponds to. Almost always
    /// zero, but the bridge is allowed to place its window anywhere in the
    /// 16-bit port range, and a driver asking for port `n` has to land at
    /// `io_cpu_base + (n - io_port_base)`.
    pub io_port_base: u64,
}

impl PciHost {
    /// True when there is an I/O window to translate port numbers into.
    pub fn has_io_window(&self) -> bool {
        self.io_size != 0
    }
}
