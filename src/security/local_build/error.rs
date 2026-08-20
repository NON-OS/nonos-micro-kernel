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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalBuildError {
    NoIdentity,
    ProofFailed,
    TrailerShape,
    /// This kernel verifies capsules with a STARK, and minting one costs
    /// minutes rather than milliseconds. A Pedersen trailer would be read as
    /// malformed at spawn, so nothing is minted.
    StarkRequired,
}

impl LocalBuildError {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoIdentity => "no local build identity",
            Self::ProofFailed => "local proof generation failed",
            Self::TrailerShape => "proof does not match the trailer layout",
            Self::StarkRequired => "this image verifies with a STARK; local signing is not wired for it",
        }
    }
}
