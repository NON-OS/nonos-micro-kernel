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

use super::cert_sig_alg::{cert_sig_alg, SigAlg};

pub fn verify_link(child: &[u8], parent_spki: &[u8]) -> bool {
    let Some(tbs) = super::cert_tbs::cert_tbs(child) else {
        return false;
    };
    let Some(sig) = super::cert_signature::cert_signature(child) else {
        return false;
    };
    let Some(alg) = cert_sig_alg(child) else {
        return false;
    };
    match alg {
        SigAlg::EcdsaP256Sha256 => ecdsa(parent_spki, tbs, sig, 32),
        SigAlg::EcdsaP384Sha384 => ecdsa(parent_spki, tbs, sig, 48),
        SigAlg::RsaPkcs1Sha256 => match super::hash_sha256::hash_sha256(tbs) {
            Some(d) => super::verify_rsa::verify_rsa(0, 0, parent_spki, sig, &d),
            None => false,
        },
        SigAlg::RsaPkcs1Sha384 => match super::hash_sha384::hash_sha384(tbs) {
            Some(d) => super::verify_rsa::verify_rsa(0, 1, parent_spki, sig, &d),
            None => false,
        },
        SigAlg::RsaPssSha256 => match super::hash_sha256::hash_sha256(tbs) {
            Some(d) => super::verify_rsa::verify_rsa(1, 0, parent_spki, sig, &d),
            None => false,
        },
        SigAlg::RsaPkcs1Sha512 => false,
    }
}

fn ecdsa(parent_spki: &[u8], tbs: &[u8], sig: &[u8], width: usize) -> bool {
    let Some(pt) = super::spki_point::spki_point(parent_spki) else {
        return false;
    };
    if width == 32 {
        if pt.len() != 65 {
            return false;
        }
        let Some(d) = super::hash_sha256::hash_sha256(tbs) else {
            return false;
        };
        let mut pk = [0u8; 65];
        pk.copy_from_slice(pt);
        let mut raw = [0u8; 64];
        super::ecdsa_sig_raw::ecdsa_sig_raw(sig, 32, &mut raw)
            && super::verify_p256::verify_p256(&pk, &raw, &d)
    } else {
        if pt.len() != 97 {
            return false;
        }
        let Some(d) = super::hash_sha384::hash_sha384(tbs) else {
            return false;
        };
        let mut pk = [0u8; 97];
        pk.copy_from_slice(pt);
        let mut raw = [0u8; 96];
        super::ecdsa_sig_raw::ecdsa_sig_raw(sig, 48, &mut raw)
            && super::verify_p384::verify_p384(&pk, &raw, &d)
    }
}
