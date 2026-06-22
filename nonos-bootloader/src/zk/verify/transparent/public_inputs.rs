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

use super::constants::{RUNTIME_BOOT_PUBLIC_INPUTS_LEN, STATIC_BOOT_PUBLIC_INPUTS_LEN};
use super::input_types::BootTransparentInputs;
use crate::zk::binding::replay::ZkPublicInputs;

pub fn public_inputs(buf: &[u8]) -> Result<BootTransparentInputs, &'static str> {
    if buf.len() == STATIC_BOOT_PUBLIC_INPUTS_LEN {
        let mut kernel_hash = [0u8; 32];
        kernel_hash.copy_from_slice(buf);
        return Ok(BootTransparentInputs::Static { kernel_hash });
    }
    if buf.len() == RUNTIME_BOOT_PUBLIC_INPUTS_LEN {
        return ZkPublicInputs::from_bytes(buf)
            .map(BootTransparentInputs::Runtime)
            .ok_or("transparent public input malformed");
    }
    Err("transparent public input size invalid")
}
