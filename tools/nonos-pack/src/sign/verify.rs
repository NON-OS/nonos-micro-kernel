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

use nonos_capsule_sign::sign::verify_with;
use nonos_capsule_sign::verify::decode::decode_cert;

use super::trailer::{parse, REQUIRED};
use crate::container::{decode, section, PackErr, SectionKind};

pub fn verify(bytes: &[u8]) -> Result<(), PackErr> {
    let (c, trailer_off) = decode(bytes)?;
    let digest = blake3::hash(&bytes[..trailer_off]);
    let cert_bytes = section(&c, SectionKind::IdCert).ok_or(PackErr::MissingSection)?;
    let cert = decode_cert(cert_bytes).map_err(|e| PackErr::BadCert(e.to_string()))?;
    let sigs = parse(&bytes[trailer_off..])?;
    for (tag, alg) in REQUIRED {
        let sig =
            sigs.iter().find(|(t, _)| *t == tag).ok_or(PackErr::MissingSignature(alg.label()))?;
        let key = cert
            .publisher_keys
            .iter()
            .find(|k| k.alg == alg)
            .ok_or(PackErr::MissingPublisherKey(alg.label()))?;
        let ok = verify_with(alg, &key.pubkey, digest.as_bytes(), sig.1)
            .map_err(|e| PackErr::Crypto(e.to_string()))?;
        if !ok {
            return Err(PackErr::BadSignature(alg.label()));
        }
    }
    Ok(())
}
