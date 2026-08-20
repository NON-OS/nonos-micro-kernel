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

//! Proving what this machine built, so it will run it.
//!
//! One secret, one leaf, one root. Each local build gets a membership proof
//! whose challenge binds the measurement and the capabilities, so a trailer
//! minted for a capsule holding nothing does not verify for the same bytes
//! installed with more.
//!
//! Nothing here enrols. Minting a proof is not consent.

mod error;
mod identity;
mod sign;
mod trailer;
mod tree;

pub use error::LocalBuildError;
pub use identity::root;
pub use sign::sign;
