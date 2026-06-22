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

mod commit;
pub mod replay;
mod select;
mod types;
pub mod verify;

pub use commit::{compute_capsule_commitment, compute_commit, verify_commitment};
pub use replay::{
    build_public_inputs, derive_machine_id, get_boot_nonce, get_boot_nonce_checked, get_machine_id,
    get_machine_id_checked, init_boot_nonce, init_boot_nonce_value, init_machine_id,
    is_machine_id_initialized, is_nonce_initialized, verify_machine_id, verify_nonce_freshness,
    ZkPublicInputs,
};
pub use select::{is_manifest_binding_enabled, select_binding};
pub use types::{BindingInput, DS_COMMITMENT, MAX_MANIFEST_SIZE};
pub use verify::{verify_kernel_binding, verify_proof_binding, BindingError};
