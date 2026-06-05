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

extern crate alloc;

use alloc::vec::Vec;
use nonos_marketplace_abi::{release_signing_bytes, MarketplaceIndex};

use super::constants::ED25519_SIG_LEN;
use crate::verify::{Verdict, Verifier};

pub(super) fn verify_publisher_signatures<V: Verifier>(
    index: &MarketplaceIndex,
    verifier: &V,
) -> Vec<bool> {
    let mut out = Vec::new();
    for entry in &index.entries {
        for release in &entry.releases {
            let ok = if entry.publisher_pubkey.iter().all(|&b| b == 0)
                || release.publisher_signature.len() != ED25519_SIG_LEN
            {
                false
            } else {
                let signed = release_signing_bytes(release);
                verifier.verify(&signed, &release.publisher_signature, &entry.publisher_pubkey)
                    == Verdict::Accepted
            };
            out.push(ok);
        }
    }
    out
}
