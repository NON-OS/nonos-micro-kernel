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

#![allow(clippy::needless_range_loop)]

mod ed25519;
mod field;
mod util;

// X25519 ECDH is reached only from the legacy onion/wifi/zkids paths
// (`crate::network::onion::*`, `crate::drivers::wifi::*`, and
// `crate::security::network::zkids::*`). The trusted-path microkernel
// build performs no X25519, and the `cfg(not(feature = "crypto-curve25519"))`
// fallback in `x25519.rs` is broken (missing imports for `FieldElement`,
// `X25519_BASEPOINT`, and `x25519_clamp`). Compile the module only when
// the dalek-backed feature is on or when the legacy tree is selected.

pub use ed25519::EdwardsPoint;
pub use field::FieldElement;

pub(crate) use util::{load_u64_le, store_u64_le, SQRT_M1};
