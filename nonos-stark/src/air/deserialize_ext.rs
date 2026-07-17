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

//! Parsing a money-grade proof from the bytes a capsule ships. It reads only from
//! untrusted input, validates every field element against the modulus, and returns
//! None on any malformed byte, so an attestation never panics on a hostile trailer.

use super::super::field::{Fp, Fp2, P};
use super::super::fri_ext::{FriProofExt, LayerOpeningExt, QueryProofExt};
use super::types_ext::{StarkProofExt, StarkQueryExt};
use alloc::vec::Vec;

struct Reader<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.i.checked_add(n)?;
        if end > self.b.len() {
            return None;
        }
        let s = &self.b[self.i..end];
        self.i = end;
        Some(s)
    }
    /// Bytes still unread. A length field is capped at this, since every element
    /// consumes at least one byte, so a hostile count can never over-allocate.
    fn remaining(&self) -> usize {
        self.b.len() - self.i
    }
    fn u32(&mut self) -> Option<usize> {
        let b = self.take(4)?;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
    }
    fn u64(&mut self) -> Option<u64> {
        let b = self.take(8)?;
        Some(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
    }
    fn fp(&mut self) -> Option<Fp> {
        let v = self.u64()?;
        (v < P).then(|| Fp::from_u64(v))
    }
    fn fp2(&mut self) -> Option<Fp2> {
        Some(Fp2 { c0: self.fp()?, c1: self.fp()? })
    }
    fn digest(&mut self) -> Option<[u8; 32]> {
        let mut d = [0u8; 32];
        d.copy_from_slice(self.take(32)?);
        Some(d)
    }
    fn path(&mut self) -> Option<Vec<[u8; 32]>> {
        let n = self.u32()?;
        let mut v = Vec::with_capacity(n.min(self.remaining()));
        for _ in 0..n {
            v.push(self.digest()?);
        }
        Some(v)
    }
    fn fp2s(&mut self) -> Option<Vec<Fp2>> {
        let n = self.u32()?;
        let mut v = Vec::with_capacity(n.min(self.remaining()));
        for _ in 0..n {
            v.push(self.fp2()?);
        }
        Some(v)
    }
}

/// Parse a money-grade proof, or None on any malformed input.
pub fn deserialize_proof_ext(bytes: &[u8]) -> Option<StarkProofExt> {
    let mut r = Reader { b: bytes, i: 0 };
    let trace_root = r.digest()?;
    let comp_root = r.digest()?;
    let ood_frame = r.fp2s()?;

    let nroots = r.u32()?;
    let mut roots = Vec::with_capacity(nroots.min(r.remaining()));
    for _ in 0..nroots {
        roots.push(r.digest()?);
    }
    let final_layer = r.fp2s()?;
    let nfq = r.u32()?;
    let mut fri_queries = Vec::with_capacity(nfq.min(r.remaining()));
    for _ in 0..nfq {
        let nl = r.u32()?;
        let mut layers = Vec::with_capacity(nl.min(r.remaining()));
        for _ in 0..nl {
            let a = r.fp2()?;
            let a_path = r.path()?;
            let b = r.fp2()?;
            let b_path = r.path()?;
            layers.push(LayerOpeningExt { a, a_path, b, b_path });
        }
        fri_queries.push(QueryProofExt { layers });
    }
    let pow_nonce = r.u64()?;
    let fri = FriProofExt { roots, final_layer, queries: fri_queries, pow_nonce };

    let nq = r.u32()?;
    let mut queries = Vec::with_capacity(nq.min(r.remaining()));
    for _ in 0..nq {
        let deep = r.fp2()?;
        let deep_path = r.path()?;
        let nt = r.u32()?;
        let mut trace = Vec::with_capacity(nt.min(r.remaining()));
        for _ in 0..nt {
            trace.push(r.fp()?);
        }
        let trace_path = r.path()?;
        let comp = r.fp2()?;
        let comp_path = r.path()?;
        queries.push(StarkQueryExt { deep, deep_path, trace, trace_path, comp, comp_path });
    }

    Some(StarkProofExt { trace_root, comp_root, ood_frame, fri, queries })
}
