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

use super::input_types::BootTransparentInputs;
use crate::zk::binding::replay::{get_boot_nonce_checked, get_machine_id_checked};
use crate::zk::verify::util::ct_eq32;

pub fn freshness(inputs: &BootTransparentInputs) -> Result<(), &'static str> {
    let BootTransparentInputs::Runtime(inputs) = inputs else {
        return Ok(());
    };
    let nonce = get_boot_nonce_checked().ok_or("boot nonce not initialized")?;
    let machine = get_machine_id_checked().ok_or("machine id not initialized")?;
    if !ct_eq32(&inputs.boot_nonce, &nonce) {
        return Err("boot nonce mismatch");
    }
    if !ct_eq32(&inputs.machine_id, &machine) {
        return Err("machine id mismatch");
    }
    Ok(())
}
