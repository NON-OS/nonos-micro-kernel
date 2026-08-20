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


use super::error::AttestDocError;
use super::produce::produce;
use crate::security::attest_doc::document::AttestationDoc;
use crate::security::tpm::ak::load_ak;

/// Answer a challenge with a signed statement of what this machine is running.
///
/// The key is derived on first use rather than at boot: a machine nobody ever
/// asks does not need to occupy a TPM object slot, and the derivation is
/// deterministic so it costs the same whenever it happens.
pub fn attest(challenge: &[u8; 32]) -> Result<AttestationDoc, AttestDocError> {
    let handle = load_ak().map_err(AttestDocError::Tpm)?;
    produce(handle, challenge)
}
