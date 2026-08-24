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

//! The gate every call that claims to confine a device passes through.
//!
//! Without it the backend writes page table entries into tables no unit walks
//! and returns success, and the DMA allocator hands the caller a buffer whose
//! bus address is an IOVA. A device given that address with translation off
//! does not fault: it treats the IOVA as a physical address and writes to
//! whatever lives there. So the failure is not a missing protection, it is
//! memory corruption, and the only safe answer before a unit is in service is
//! to refuse and let the caller take the untranslated DMA path.

use crate::arch::x86_64::iommu::globals::is_enforcing;
use crate::memory::iommu::IommuError;

pub(super) fn require() -> Result<(), IommuError> {
    if is_enforcing() {
        Ok(())
    } else {
        Err(IommuError::NotInitialized)
    }
}
