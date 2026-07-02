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

pub mod args;
pub mod constants;
pub mod ed25519_result;
pub mod footer;
pub mod key_id_ed25519;
pub mod key_id_mldsa65;
pub mod load_mldsa65_pub;
pub mod message;
pub mod print_summary;
pub mod release_signature_blob;
pub mod sign_ed25519;
pub mod sign_ed25519_vault;
pub mod sign_mldsa65;
pub mod vault;
pub mod verify_signed_kernel;
pub mod write_signed_kernel;

pub use args::Args;
pub use load_mldsa65_pub::load_mldsa65_pub;
pub use message::signed_message;
pub use print_summary::print_summary;
pub use release_signature_blob::release_signature_blob;
pub use sign_ed25519::sign_ed25519;
pub use sign_mldsa65::sign_mldsa65;
pub use vault::{sign_kernel_with_vault, VaultClient, VaultError, VAULT_TIMEOUT_SECS};
pub use verify_signed_kernel::verify_signed_kernel;
pub use write_signed_kernel::write_signed_kernel;
