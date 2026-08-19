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

// The real VT-d capability decode and table encoding the kernel uses.
#[allow(dead_code)]
#[path = "../../../../src/arch/x86_64/iommu/regs/cap/mod.rs"]
pub mod cap;

#[allow(dead_code)]
#[path = "../../../../src/arch/x86_64/iommu/tables/sl_pte.rs"]
pub mod sl_pte;

#[allow(dead_code)]
#[path = "../../../../src/arch/x86_64/iommu/tables/context.rs"]
pub mod context;
