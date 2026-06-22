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

use uefi::prelude::*;
use uefi::table::runtime::VariableVendor;

pub fn read_pending_nonce(st: &SystemTable<Boot>) -> Option<[u8; 32]> {
    let mut out = [0u8; 32];
    st.runtime_services()
        .get_variable(
            uefi::cstr16!("NonosZkPendingNonce"),
            &VariableVendor::GLOBAL_VARIABLE,
            &mut out,
        )
        .ok()?;
    Some(out)
}

pub fn has_pending_challenge(st: &SystemTable<Boot>) -> bool {
    read_pending_nonce(st).is_some()
}
