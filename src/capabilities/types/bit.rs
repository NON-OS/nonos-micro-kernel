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

use super::Capability;

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
            Self::SpawnWindow => 16777216,
            Self::ProcessControl => 33554432,
            Self::StoreWrite => 67108864,
        }
    }
}
