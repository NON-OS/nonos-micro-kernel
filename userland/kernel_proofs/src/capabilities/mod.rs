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

// The real capability enum and token type. The token carries pure grant/expiry
// logic (no signature check), so tokens can be constructed directly for the
// proofs; the signing/nonce/revocation machinery is not needed here.
#[path = "../../../../src/capabilities/types/mod.rs"]
pub mod types;

// Style lints below are the real token code's own choices.
#[allow(clippy::manual_contains, clippy::unnecessary_map_or)]
#[path = "../../../../src/capabilities/token/types/mod.rs"]
mod token_types;

pub use token_types::CapabilityToken;
pub use types::Capability;

// The real bit-token operations: has/add/remove and the bits<->caps
// conversions the kernel authorization path executes.
#[path = "../../../../src/capabilities/bits.rs"]
pub mod bits;

// The single-bit mask a capability occupies. `Capability::bit` is `pub(crate)`
// in the kernel; expose it here so the bit-distinctness proof exercises it.
pub fn bit_of(cap: Capability) -> u64 {
    cap.bit()
}
