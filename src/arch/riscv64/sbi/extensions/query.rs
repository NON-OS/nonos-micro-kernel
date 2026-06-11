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

use crate::arch::riscv64::sbi::base::probe_extension_base;
use crate::arch::riscv64::sbi::SbiError;

use super::extension::Extension;

pub fn probe_extension(ext: Extension) -> Result<bool, SbiError> {
    probe_extension_base(ext.eid())
}

pub fn has_timer() -> Result<bool, SbiError> {
    probe_extension(Extension::Timer)
}
pub fn has_ipi() -> Result<bool, SbiError> {
    probe_extension(Extension::Ipi)
}
pub fn has_rfence() -> Result<bool, SbiError> {
    probe_extension(Extension::Rfence)
}
pub fn has_hsm() -> Result<bool, SbiError> {
    probe_extension(Extension::Hsm)
}
pub fn has_srst() -> Result<bool, SbiError> {
    probe_extension(Extension::Srst)
}
pub fn has_pmu() -> Result<bool, SbiError> {
    probe_extension(Extension::Pmu)
}
pub fn has_dbcn() -> Result<bool, SbiError> {
    probe_extension(Extension::Dbcn)
}
pub fn has_susp() -> Result<bool, SbiError> {
    probe_extension(Extension::Susp)
}
