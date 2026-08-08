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

pub use super::ops_helpers::gsi_for_irq;
pub use super::ops_msi::{claim_gsi_for_msi, release_gsi_from_msi};
pub use super::ops_query::{query, restore, snapshot};
pub use super::ops_route::{
    alloc_route, free_vector, mask, program_route, program_route_external, retarget,
};
pub use super::ops_status::{status, IoApicStatus};
