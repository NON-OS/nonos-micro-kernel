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

mod machine;
mod nonce;
mod public_inputs;

pub use machine::{
    derive_machine_id, get_machine_id, get_machine_id_checked, init_machine_id,
    is_machine_id_initialized, verify_machine_id,
};
pub use nonce::{
    get_boot_nonce, get_boot_nonce_checked, init_boot_nonce, init_boot_nonce_value,
    is_nonce_initialized, verify_nonce_freshness,
};
pub use public_inputs::{build_public_inputs, ZkPublicInputs};
