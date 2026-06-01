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

use core::ptr;

use super::state::TableRegistry;
use crate::arch::x86_64::acpi::error::{AcpiError, AcpiResult};
use crate::arch::x86_64::acpi::tables::{Fadt, SIG_FADT};

pub fn parse_fadt(registry: &mut TableRegistry) -> AcpiResult<()> {
    let addr = *registry.tables.get(&SIG_FADT).ok_or(AcpiError::FadtNotFound)?;
    let addr = super::phys::directmap(addr).ok_or(AcpiError::FadtNotFound)?;

    unsafe {
        let fadt = ptr::read_volatile(addr as *const Fadt);

        registry.data.pm1a_control = fadt.pm1a_control_block;
        registry.data.pm1b_control = fadt.pm1b_control_block;
        registry.data.pm_profile = fadt.pm_profile();
        registry.data.sci_interrupt = fadt.sci_interrupt;

        if fadt.has_reset_register() {
            registry.data.reset_reg = Some(fadt.reset_reg);
            registry.data.reset_value = fadt.reset_value;
        }

        registry.data.slp_typ[5] = 0;
    }

    Ok(())
}
