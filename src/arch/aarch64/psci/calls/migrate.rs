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

use super::super::error::PsciError;
use super::super::function::PSCI_MIGRATE_INFO_TYPE;
use super::super::raw::psci_call0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrateType {
    SingleCore,
    SingleCoreNotUp,
    NotRequired,
}

pub fn migrate_info_type() -> Result<MigrateType, PsciError> {
    let ret = psci_call0(PSCI_MIGRATE_INFO_TYPE);
    if ret < 0 {
        PsciError::from_ret(ret as i32)?;
    }
    match ret {
        0 => Ok(MigrateType::SingleCore),
        1 => Ok(MigrateType::SingleCoreNotUp),
        2 => Ok(MigrateType::NotRequired),
        _ => Err(PsciError::InvalidParams),
    }
}
