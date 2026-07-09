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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    CoreExec,
    IO,
    Network,
    IPC,
    Memory,
    Crypto,
    FileSystem,
    Hardware,
    Debug,
    Admin,
    RegisterService,
    GraphicsDisplayQuery,
    GraphicsSurfaceCreate,
    GraphicsSurfaceMap,
    GraphicsPresent,
    DeviceEnum,
    // Driver-broker authority. `DeviceEnum` is enumerate-only;
    // `Driver` lets a capsule claim and release a device; `Mmio`
    // lets a claim holder map a slice of a BAR into its own AS;
    // `Irq` lets a claim holder bind a device interrupt to a
    // kernel-delivered notification slot; `Dma` lets a claim
    // holder receive a DMA-coherent buffer the device is allowed
    // to read or write through; `Pio` lets a claim holder mint a
    // PIO grant against a port BAR and execute kernel-mediated
    // `in`/`out` instructions on its ports.
    Driver,
    Mmio,
    Irq,
    Dma,
    Pio,
    InputSource,
    TimeSet,
    // Grants a spawn site the authority to attribute a capsule-load's
    // parent to a kernel-attested pid other than the caller itself
    // (see `on_behalf_of` on `CapsuleLoadRequest`). No ordinary capsule
    // manifest declares this bit; only the installer's does.
    SpawnBroker,
}

impl Capability {
    #[inline]
    pub(crate) const fn bit(self) -> u64 {
        match self {
            Self::CoreExec => 1,
            Self::IO => 2,
            Self::Network => 4,
            Self::IPC => 8,
            Self::Memory => 16,
            Self::Crypto => 32,
            Self::FileSystem => 64,
            Self::Hardware => 128,
            Self::Debug => 256,
            Self::Admin => 512,
            Self::RegisterService => 1024,
            Self::GraphicsDisplayQuery => 2048,
            Self::GraphicsSurfaceCreate => 4096,
            Self::GraphicsSurfaceMap => 8192,
            Self::GraphicsPresent => 16384,
            Self::DeviceEnum => 32768,
            Self::Driver => 65536,
            Self::Mmio => 131072,
            Self::Irq => 262144,
            Self::Dma => 524288,
            Self::Pio => 1048576,
            Self::InputSource => 2097152,
            Self::TimeSet => 4194304,
            Self::SpawnBroker => 8388608,
        }
    }

    pub const fn all() -> [Capability; 24] {
        [
            Self::CoreExec,
            Self::IO,
            Self::Network,
            Self::IPC,
            Self::Memory,
            Self::Crypto,
            Self::FileSystem,
            Self::Hardware,
            Self::Debug,
            Self::Admin,
            Self::RegisterService,
            Self::GraphicsDisplayQuery,
            Self::GraphicsSurfaceCreate,
            Self::GraphicsSurfaceMap,
            Self::GraphicsPresent,
            Self::DeviceEnum,
            Self::Driver,
            Self::Mmio,
            Self::Irq,
            Self::Dma,
            Self::Pio,
            Self::InputSource,
            Self::TimeSet,
            Self::SpawnBroker,
        ]
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::CoreExec => "CoreExec",
            Self::IO => "IO",
            Self::Network => "Network",
            Self::IPC => "IPC",
            Self::Memory => "Memory",
            Self::Crypto => "Crypto",
            Self::FileSystem => "FileSystem",
            Self::Hardware => "Hardware",
            Self::Debug => "Debug",
            Self::Admin => "Admin",
            Self::RegisterService => "RegisterService",
            Self::GraphicsDisplayQuery => "GraphicsDisplayQuery",
            Self::GraphicsSurfaceCreate => "GraphicsSurfaceCreate",
            Self::GraphicsSurfaceMap => "GraphicsSurfaceMap",
            Self::GraphicsPresent => "GraphicsPresent",
            Self::DeviceEnum => "DeviceEnum",
            Self::Driver => "Driver",
            Self::Mmio => "Mmio",
            Self::Irq => "Irq",
            Self::Dma => "Dma",
            Self::Pio => "Pio",
            Self::InputSource => "InputSource",
            Self::TimeSet => "TimeSet",
            Self::SpawnBroker => "SpawnBroker",
        }
    }

    pub const fn count() -> usize {
        23
    }
}

impl core::fmt::Display for Capability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
