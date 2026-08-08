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

pub mod constants;
pub mod error;
pub mod gsi_owners;
pub mod init;
pub mod init_from_acpi;
pub mod mmio;
pub mod ops;
mod ops_helpers;
mod ops_msi;
mod ops_query;
mod ops_route;
mod ops_status;
pub mod state;
mod state_alloc;
mod state_chip;
pub mod types;
mod types_madt;
mod types_rte;

pub use error::{IoApicError, IoApicResult};
pub use gsi_owners::{
    claim_for_capsule as claim_gsi_for_capsule, claim_for_kernel as claim_gsi_for_kernel,
    owner_of as gsi_owner_of, release_capsule as release_gsi_from_capsule,
};
pub use init::init;
pub use init_from_acpi::init_from_acpi;
pub use ops::{
    alloc_route, claim_gsi_for_msi, free_vector, gsi_for_irq, mask, program_route,
    program_route_external, query, release_gsi_from_msi, restore, retarget, snapshot, status,
    IoApicStatus,
};
pub use state::{count, is_initialized};
pub use types::{IsoFlags, MadtIoApic, MadtIso, MadtNmi, Rte};
