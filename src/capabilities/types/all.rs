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
    pub const fn all() -> [Capability; 27] {
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
            Self::SpawnWindow,
            Self::ProcessControl,
            Self::StoreWrite,
        ]
    }

    pub const fn count() -> usize {
        23
    }
}
