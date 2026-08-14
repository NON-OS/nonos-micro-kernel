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

use alloc::vec::Vec;

use super::binding::qualifying_data;
use super::document::AttestationDoc;
use super::error::AttestDocError;
use crate::security::attest_registry::{attested_count, registry_complete, registry_root};
use crate::security::tpm::crb::transact;
use crate::security::tpm::quote::{build_quote, check_attest, parse_quote};

/// PCRs covered by the quote: the firmware and boot chain measurements the
/// bootloader extended. Naming them explicitly rather than quoting every PCR
/// keeps the document meaningful, since a verifier has to know which values it
/// is being shown.
const QUOTED_PCRS: [u8; 4] = [0, 1, 2, 7];

/// Largest response this driver will accept from the part.
const RESPONSE_MAX: usize = 4096;

/// Answer "what is this machine running", signed.
///
/// Refuses when the registry is incomplete. A document that omits a running
/// capsule is the one failure a remote party cannot detect, so the machine
/// declines to speak rather than understate itself.
pub(super) fn produce(ak_handle: u32, challenge: &[u8; 32]) -> Result<AttestationDoc, AttestDocError> {
    if !registry_complete() {
        return Err(AttestDocError::RegistryIncomplete);
    }
    let root = registry_root();
    let qualifying = qualifying_data(challenge, &root);

    let cmd = build_quote(ak_handle, &qualifying, &QUOTED_PCRS);
    let mut buf = [0u8; RESPONSE_MAX];
    // SAFETY: eK@nonos.systems - a quote reads PCR state and signs it. It
    // creates no objects and changes no key material.
    let len = unsafe { transact(&cmd, &mut buf) }.map_err(AttestDocError::Tpm)?;

    let quote = parse_quote(&buf[..len]).map_err(AttestDocError::Quote)?;
    // Checked here, not only by the verifier: a machine that would hand out a
    // quote whose nonce it never confirmed has no idea what it just signed.
    check_attest(quote.attest, &qualifying).map_err(AttestDocError::Quote)?;

    Ok(AttestationDoc {
        challenge: *challenge,
        registry_root: root,
        capsule_count: attested_count() as u32,
        registry_complete: true,
        attest: Vec::from(quote.attest),
        signature: Vec::from(quote.signature),
    })
}
