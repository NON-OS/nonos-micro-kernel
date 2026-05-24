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

// Older application-side vault. The real vault now lives at `src/vault` —
// secret custody, sealing, key lifecycle, and trust policy belong there.
// This tree only stays around because the crypto syscall in
// `src/crypto/core/syscall.rs` and the shell vault commands under
// `src/shell/commands/vault/*` still reach in for `init_vault`,
// `store_key`, `retrieve_key`, `get_signing_key`, `get_public_key`, and
// the rest. Wave 6 is where those callers get moved to
// `crate::vault::nonos_vault::*`; once that lands, this whole directory
// should be deleted.
//
// A few internals here shouldn't survive the migration. A vault has no
// business owning its own RNG (`random::generate_random_bytes` duplicates
// `crate::crypto::util::rng`) or its own secure-memory allocator
// (`crate::memory::secure_memory` is the real one). Zeroization should
// route through canonical primitives. The `MemoryType` and key-type names
// here also clash with definitions elsewhere and need to collapse.
//
// Don't extend the public API here, don't add new key-handling logic.
// Edits should only land as part of moving callers off this tree.

mod key_vault;
mod random;
mod string_vault;
mod types;
mod zeroize;

pub use key_vault::{
    delete_vault_key, generate_and_store_ed25519_keypair, get_public_key, get_signing_key,
    list_vault_keys,
};
pub use random::generate_random_bytes;
pub use string_vault::{init_vault, list_keys, retrieve_key, store_key};
pub use zeroize::zeroize_all_keys;
