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

pub fn verify_ecdsa(parent_spki: &[u8], tbs: &[u8], sig: &[u8], width: usize) -> bool {
    let Some(pt) = super::super::spki_point::spki_point(parent_spki) else {
        return false;
    };
    if width == 32 {
        let Some(d) = super::super::hash_sha256::hash_sha256(tbs) else {
            return false;
        };
        let mut pk = [0u8; 65];
        if pt.len() != pk.len() {
            return false;
        }
        pk.copy_from_slice(pt);
        let mut raw = [0u8; 64];
        super::super::ecdsa_sig_raw::ecdsa_sig_raw(sig, 32, &mut raw)
            && super::super::verify_p256::verify_p256(&pk, &raw, &d)
    } else {
        let Some(d) = super::super::hash_sha384::hash_sha384(tbs) else {
            return false;
        };
        let mut pk = [0u8; 97];
        if pt.len() != pk.len() {
            return false;
        }
        pk.copy_from_slice(pt);
        let mut raw = [0u8; 96];
        super::super::ecdsa_sig_raw::ecdsa_sig_raw(sig, 48, &mut raw)
            && super::super::verify_p384::verify_p384(&pk, &raw, &d)
    }
}
