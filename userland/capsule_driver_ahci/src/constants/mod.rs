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

pub mod ata;
pub mod pci;
pub mod port;
pub mod regs;

pub use pci::{AHCI_ABAR_BAR, CLASS_BLOCK};
pub use port::{MAX_PORTS, PORT_KIND_NONE, PORT_KIND_SATA};
pub use regs::{
    GHC_AE, GHC_HR, HBA_CAP, HBA_CAP2, HBA_GHC, HBA_IS, HBA_PI, HBA_VS, SIG_ATAPI, SIG_PM,
    SIG_SATA, SIG_SEMB,
};
