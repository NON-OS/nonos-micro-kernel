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

// Validate the server's TLS Certificate(11) message body for `host` at time
// `now`: the leaf must match the hostname and be in its validity window, each
// certificate must be signed by the next one up, and the top certificate's
// SubjectPublicKeyInfo must hash to a trusted root. Fail closed everywhere.
pub fn verify_chain(body: &[u8], host: &[u8], now: u64) -> bool {
    let n = super::cert_count::cert_count(body);
    if n < 1 {
        return false;
    }
    let Some(leaf) = super::cert_at::cert_at(body, 0) else {
        return false;
    };
    if !super::cert_dns_match::matches(leaf, host) || !super::cert_valid_now::cert_valid_now(leaf, now) {
        return false;
    }
    let mut i = 0u8;
    while i + 1 < n {
        let Some(child) = super::cert_at::cert_at(body, i) else {
            return false;
        };
        let Some(parent) = super::cert_at::cert_at(body, i + 1) else {
            return false;
        };
        let Some(parent_spki) = super::cert_spki::cert_spki(parent) else {
            return false;
        };
        if !super::verify_link::verify_link(child, parent_spki)
            || !super::cert_valid_now::cert_valid_now(parent, now)
        {
            return false;
        }
        i += 1;
    }
    let Some(top) = super::cert_at::cert_at(body, n - 1) else {
        return false;
    };
    let Some(top_spki) = super::cert_spki::cert_spki(top) else {
        return false;
    };
    match super::hash_sha256::hash_sha256(top_spki) {
        Some(h) => super::roots::is_trusted_spki_hash(&h),
        None => false,
    }
}
