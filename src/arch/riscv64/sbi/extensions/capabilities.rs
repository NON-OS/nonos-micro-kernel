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

use crate::arch::riscv64::sbi::SbiError;

use super::query;

#[derive(Debug, Clone)]
pub struct SbiCapabilities {
    pub timer: bool,
    pub ipi: bool,
    pub rfence: bool,
    pub hsm: bool,
    pub srst: bool,
    pub pmu: bool,
    pub dbcn: bool,
    pub susp: bool,
}

impl SbiCapabilities {
    pub fn discover() -> Result<Self, SbiError> {
        Ok(Self {
            timer: query::has_timer()?,
            ipi: query::has_ipi()?,
            rfence: query::has_rfence()?,
            hsm: query::has_hsm()?,
            srst: query::has_srst()?,
            pmu: query::has_pmu()?,
            dbcn: query::has_dbcn()?,
            susp: query::has_susp()?,
        })
    }
}
