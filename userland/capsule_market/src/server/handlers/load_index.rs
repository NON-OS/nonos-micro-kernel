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

use crate::ingest::{load_verified, IngestError};
use crate::protocol::{Request, E_INVAL, E_KEYREJECTED, E_STALE};
use crate::server::error::reply_status;
use crate::store::Store;
use crate::verify::Verifier;

pub(crate) fn handle<V: Verifier>(
    store: &mut Store,
    verifier: &V,
    body: &[u8],
    req: &Request,
    tx: &mut [u8],
) {
    let last_serial = store.last_serial();
    match load_verified(body, verifier, last_serial) {
        Ok(verified) => {
            store.install(
                verified.index,
                verified.signature_verified,
                verified.publisher_signature_verified,
            );
            reply_status(tx, req, 0);
        }
        Err(IngestError::Malformed) => reply_status(tx, req, E_INVAL),
        Err(IngestError::StaleSerial) => reply_status(tx, req, E_STALE),
        Err(IngestError::SignatureRefused) => reply_status(tx, req, E_KEYREJECTED),
        Err(IngestError::UntrustedOperator) => reply_status(tx, req, E_KEYREJECTED),
    }
}
