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

#[derive(Debug, Clone, Copy)]
pub struct TranslationFault {
    pub fault_status: u8,
    pub stage2: bool,
    pub ptw_fault: bool,
}

impl TranslationFault {
    pub fn is_translation_fault(&self) -> bool {
        (self.fault_status & 0x3C) == 0x04
    }

    pub fn is_access_fault(&self) -> bool {
        (self.fault_status & 0x3C) == 0x08
    }

    pub fn is_permission_fault(&self) -> bool {
        (self.fault_status & 0x3C) == 0x0C
    }

    pub fn level(&self) -> u8 {
        self.fault_status & 0x03
    }
}

pub(super) fn parse_par(par: u64, virt: u64) -> Result<u64, TranslationFault> {
    if par & 1 != 0 {
        return Err(TranslationFault {
            fault_status: ((par >> 1) & 0x3F) as u8,
            stage2: (par >> 9) & 1 != 0,
            ptw_fault: (par >> 8) & 1 != 0,
        });
    }
    Ok((par & 0x0000_FFFF_FFFF_F000) | (virt & 0xFFF))
}
