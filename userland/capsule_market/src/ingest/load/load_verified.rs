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

use nonos_marketplace_abi::decode_index;

use super::verify_publisher_signatures::verify_publisher_signatures;
use super::Verified;
use crate::bootstrap_trust;
use crate::ingest::IngestError;
use crate::verify::{Verdict, Verifier};

pub fn load_verified<V: Verifier>(
    blob: &[u8],
    verifier: &V,
    last_serial: u64,
) -> Result<Verified, IngestError> {
    let decoded = decode_index(blob).map_err(|_| IngestError::Malformed)?;
    if decoded.index.serial <= last_serial && last_serial != 0 {
        return Err(IngestError::StaleSerial);
    }
    if !bootstrap_trust::is_trusted(&decoded.index.operator_pubkey) {
        return Err(IngestError::UntrustedOperator);
    }
    let verdict = verifier.verify(
        decoded.signed_bytes,
        &decoded.index.index_signature,
        &decoded.index.operator_pubkey,
    );
    if verdict != Verdict::Accepted {
        return Err(IngestError::SignatureRefused);
    }
    let publisher_signature_verified = verify_publisher_signatures(&decoded.index, verifier);
    Ok(Verified { index: decoded.index, signature_verified: true, publisher_signature_verified })
}
