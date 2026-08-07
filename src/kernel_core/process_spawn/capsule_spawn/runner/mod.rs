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

mod attest_gate;
mod install;
#[cfg(feature = "nonos-dev-unverified-capsules")]
mod legacy;
pub(crate) mod preflight;
mod publisher_gate;
mod tier;
mod verified;

#[cfg(feature = "nonos-dev-unverified-capsules")]
pub use legacy::spawn;
pub use verified::spawn_verified;
pub(crate) use verified::spawn_verified_as;
pub(crate) use tier::{classify as classify_tier, Tier};
