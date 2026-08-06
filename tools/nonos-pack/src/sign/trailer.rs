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

use nonos_capsule_sign::algs::AlgId;

use crate::container::PackErr;

pub const ED25519_TAG: u8 = 1;
pub const MLDSA65_TAG: u8 = 2;

pub const REQUIRED: [(u8, AlgId); 2] =
    [(ED25519_TAG, AlgId::Ed25519), (MLDSA65_TAG, AlgId::MlDsa65)];

pub fn append(out: &mut Vec<u8>, sigs: &[(u8, Vec<u8>)]) -> Result<(), PackErr> {
    out.push(sigs.len() as u8);
    for (tag, sig) in sigs {
        let len = u16::try_from(sig.len()).map_err(|_| PackErr::SignatureTooLong)?;
        out.push(*tag);
        out.extend_from_slice(&len.to_be_bytes());
        out.extend_from_slice(sig);
    }
    Ok(())
}

pub fn parse(t: &[u8]) -> Result<Vec<(u8, &[u8])>, PackErr> {
    let count = *t.first().ok_or(PackErr::NoTrailer)? as usize;
    if count != REQUIRED.len() {
        return Err(PackErr::NonCanonicalTrailer);
    }
    let mut p = 1usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        if t.len() - p < 3 {
            return Err(PackErr::Truncated);
        }
        let tag = t[p];
        let len = u16::from_be_bytes([t[p + 1], t[p + 2]]) as usize;
        p += 3;
        if t.len() - p < len {
            return Err(PackErr::Truncated);
        }
        out.push((tag, &t[p..p + len]));
        p += len;
    }
    if p != t.len() {
        return Err(PackErr::Truncated);
    }
    if out.iter().zip(REQUIRED.iter()).any(|((tag, _), (want, _))| tag != want) {
        return Err(PackErr::NonCanonicalTrailer);
    }
    Ok(out)
}
