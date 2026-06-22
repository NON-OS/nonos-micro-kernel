// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::zk::init_machine_id;
use uefi::prelude::*;

pub fn init_fallback_machine_id(st: &SystemTable<Boot>) -> Result<(), &'static str> {
    let mut fallback = [0u8; 64];
    let vendor = st.firmware_vendor();
    for (i, ch) in vendor.iter().take(32).enumerate() {
        fallback[i] = u16::from(*ch) as u8;
    }
    let rev = st.firmware_revision();
    fallback[32..36].copy_from_slice(&rev.to_le_bytes());
    init_machine_id(&fallback);
    Ok(())
}
