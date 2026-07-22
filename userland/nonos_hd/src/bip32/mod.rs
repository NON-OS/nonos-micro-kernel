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

//! BIP32 hierarchical derivation over secp256k1. Hardened children need only
//! the parent private key; non-hardened children take the parent's compressed
//! public key from the caller, so this crate never implements point
//! multiplication itself.

mod child;
mod compress;
mod master;
mod scalar;
mod xprv;

pub use child::{child_hardened, child_normal};
pub use compress::compress_pubkey;
pub use master::master_from_seed;
pub use scalar::{add_mod_n, is_valid_scalar, ORDER};
pub use xprv::Xprv;

/// Index offset that marks a hardened derivation step.
pub const HARDENED: u32 = 0x8000_0000;
