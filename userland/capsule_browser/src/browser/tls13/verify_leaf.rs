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

pub fn verify_leaf(leaf: &[u8], issuer: &[u8]) -> bool {
    let Some(tbs) = super::cert_tbs::cert_tbs(leaf) else { return false };
    let Some(sig_der) = super::cert_signature::cert_signature(leaf) else { return false };
    let Some(spki) = super::cert_spki::cert_spki(issuer) else { return false };
    let Some(point) = super::spki_point::spki_point(spki) else { return false };
    let Some(digest) = super::hash_sha256::hash_sha256(tbs) else { return false };
    if point.len() != 65 {
        return false;
    }
    let mut pk = [0u8; 65];
    let mut sig = [0u8; 64];
    pk.copy_from_slice(point);
    if !super::ecdsa_sig_raw::ecdsa_sig_raw(sig_der, 32, &mut sig) {
        return false;
    }
    super::verify_p256::verify_p256(&pk, &sig, &digest)
}
