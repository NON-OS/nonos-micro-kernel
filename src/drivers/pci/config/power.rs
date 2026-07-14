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

use super::super::error::Result;
use super::config_space::ConfigSpace;

impl ConfigSpace {
    pub fn find_pm_capability(&self) -> Result<Option<u8>> {
        if !self.has_capabilities()? {
            return Ok(None);
        }

        let mut cap_ptr = self.capabilities_pointer()? & 0xFC;
        let mut iterations = 0;

        while cap_ptr != 0 && iterations < 48 {
            let cap_id = self.read8(cap_ptr as u16)?;
            if cap_id == 0x01 {
                return Ok(Some(cap_ptr));
            }
            cap_ptr = self.read8((cap_ptr + 1) as u16)? & 0xFC;
            iterations += 1;
        }

        Ok(None)
    }

    pub fn set_power_state_d0(&self) -> Result<()> {
        if let Some(pm_cap) = self.find_pm_capability()? {
            let pmcsr_offset = (pm_cap + 4) as u16;
            let pmcsr = self.read16(pmcsr_offset)?;
            if pmcsr & 0x0003 == 0 {
                return Ok(());
            }

            let new_pmcsr = pmcsr & !0x0003;
            self.write16(pmcsr_offset, new_pmcsr)?;

            // PCI PM requires 10 ms of recovery after leaving D3hot before the
            // function is accessed; a shorter spin lets a still-recovering
            // device misread during bring-up.
            crate::sys::timer::delay_ms(10);
        }
        Ok(())
    }

    pub fn get_power_state(&self) -> Result<u8> {
        if let Some(pm_cap) = self.find_pm_capability()? {
            let pmcsr_offset = (pm_cap + 4) as u16;
            let pmcsr = self.read16(pmcsr_offset)?;
            Ok((pmcsr & 0x0003) as u8)
        } else {
            Ok(0)
        }
    }
}
