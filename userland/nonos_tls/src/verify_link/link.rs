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

use super::super::cert_sig_alg::{cert_sig_alg, SigAlg};
use crate::verify_link::{verify_ecdsa, verify_rsa};

pub fn verify_link(child: &[u8], parent_spki: &[u8]) -> bool {
    let Some(tbs) = super::super::cert_tbs::cert_tbs(child) else {
        return false;
    };
    let Some(sig) = super::super::cert_signature::cert_signature(child) else {
        return false;
    };
    match cert_sig_alg(child) {
        Some(SigAlg::EcdsaP256Sha256) => verify_ecdsa::verify_ecdsa(parent_spki, tbs, sig, 32),
        Some(SigAlg::EcdsaP384Sha384) => verify_ecdsa::verify_ecdsa(parent_spki, tbs, sig, 48),
        Some(SigAlg::RsaPkcs1Sha256) => verify_rsa::verify_rsa(0, 0, parent_spki, tbs, sig),
        Some(SigAlg::RsaPkcs1Sha384) => verify_rsa::verify_rsa(0, 1, parent_spki, tbs, sig),
        Some(SigAlg::RsaPssSha256) => verify_rsa::verify_rsa(1, 0, parent_spki, tbs, sig),
        Some(SigAlg::RsaPkcs1Sha512) | None => false,
    }
}
