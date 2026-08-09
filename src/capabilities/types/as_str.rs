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
            Self::SpawnWindow => "SpawnWindow",
            Self::ProcessControl => "ProcessControl",
            Self::StoreWrite => "StoreWrite",
        }
    }
}
