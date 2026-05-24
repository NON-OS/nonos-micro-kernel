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

use crate::arch::aarch64::cpu::features::{has_feature, CpuFeature};

use super::control::enable_pac;
use super::error::PacResult;
use super::keygen::generate_keys;
use super::registers::install_keys;

pub fn init_pac() -> PacResult<()> {
    if !has_feature(CpuFeature::Pauth) {
        return Ok(());
    }
    let keys = generate_keys()?;
    install_keys(&keys);
    enable_pac();
    Ok(())
}
