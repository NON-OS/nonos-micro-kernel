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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SigAlg {
    EcdsaP256Sha256,
    EcdsaP384Sha384,
    RsaPkcs1Sha256,
    RsaPkcs1Sha384,
    RsaPkcs1Sha512,
    RsaPssSha256,
}

pub fn cert_sig_alg(cert: &[u8]) -> Option<SigAlg> {
    let (_, val, _) = super::der_tlv::der_tlv(cert, 0)?;
    let (_, _, tbs_end) = super::der_tlv::der_tlv(cert, val)?;
    let (tag, sa_val, _) = super::der_tlv::der_tlv(cert, tbs_end)?;
    if tag != 0x30 {
        return None;
    }
    let (otag, ov, oe) = super::der_tlv::der_tlv(cert, sa_val)?;
    if otag != 0x06 {
        return None;
    }
    match &cert[ov..oe] {
        [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02] => Some(SigAlg::EcdsaP256Sha256),
        [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x03] => Some(SigAlg::EcdsaP384Sha384),
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B] => Some(SigAlg::RsaPkcs1Sha256),
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0C] => Some(SigAlg::RsaPkcs1Sha384),
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0D] => Some(SigAlg::RsaPkcs1Sha512),
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0A] => Some(SigAlg::RsaPssSha256),
        _ => None,
    }
}
