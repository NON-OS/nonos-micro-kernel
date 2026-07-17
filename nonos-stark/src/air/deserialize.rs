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

//! Decoding the canonical proof encoding. Reads attacker-supplied bytes: total,
//! never panics, allocates only what the input can back, and rejects a
//! non-canonical field element or trailing bytes, so the round trip is exact.

use super::super::field::{Fp, P};
use super::super::fri::{FriProof, LayerOpening, QueryProof};
use super::types::{StarkProof, StarkQuery};
use alloc::vec::Vec;

/// A cursor that yields `None` past the end.
struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Reader<'a> {
        Reader { bytes, pos: 0 }
    }

    fn remaining(&self) -> usize {
        self.bytes.len() - self.pos
    }

    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.bytes.get(self.pos..self.pos + n)?;
        self.pos += n;
        Some(s)
    }

    fn len(&mut self) -> Option<usize> {
        let b = self.take(4)?;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
    }

    fn fp(&mut self) -> Option<Fp> {
        let b = self.take(8)?;
        let v = u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
        (v < P).then(|| Fp::from_u64(v))
    }

    fn digest(&mut self) -> Option<[u8; 32]> {
        let mut d = [0u8; 32];
        d.copy_from_slice(self.take(32)?);
        Some(d)
    }

    fn digests(&mut self) -> Option<Vec<[u8; 32]>> {
        let n = self.len()?;
        if n > self.remaining() / 32 {
            return None;
        }
        (0..n).map(|_| self.digest()).collect()
    }

    fn fps(&mut self) -> Option<Vec<Fp>> {
        let n = self.len()?;
        if n > self.remaining() / 8 {
            return None;
        }
        (0..n).map(|_| self.fp()).collect()
    }

    /// A length prefix is at least a four-byte word per item, so cap by that
    /// before looping and no crafted count can over-allocate.
    fn count(&mut self) -> Option<usize> {
        let n = self.len()?;
        (n <= self.remaining() / 4).then_some(n)
    }
}

fn read_layer(r: &mut Reader) -> Option<LayerOpening> {
    let a = r.fp()?;
    let a_path = r.digests()?;
    let b = r.fp()?;
    let b_path = r.digests()?;
    Some(LayerOpening { a, a_path, b, b_path })
}

fn read_fri(r: &mut Reader) -> Option<FriProof> {
    let roots = r.digests()?;
    let final_layer = r.fps()?;
    let n = r.count()?;
    let mut queries = Vec::with_capacity(n);
    for _ in 0..n {
        let m = r.count()?;
        let layers = (0..m).map(|_| read_layer(r)).collect::<Option<_>>()?;
        queries.push(QueryProof { layers });
    }
    Some(FriProof { roots, final_layer, queries })
}

fn read_query(r: &mut Reader) -> Option<StarkQuery> {
    let deep = r.fp()?;
    let deep_path = r.digests()?;
    let trace = r.fps()?;
    let n = r.count()?;
    let trace_paths = (0..n).map(|_| r.digests()).collect::<Option<_>>()?;
    let comp = r.fp()?;
    let comp_path = r.digests()?;
    Some(StarkQuery { deep, deep_path, trace, trace_paths, comp, comp_path })
}

/// Decode a proof, or `None` if the bytes are not a canonical proof.
pub fn deserialize_proof(bytes: &[u8]) -> Option<StarkProof> {
    let mut r = Reader::new(bytes);
    let trace_roots = r.digests()?;
    let comp_root = r.digest()?;
    let ood_frame = r.fps()?;
    let fri = read_fri(&mut r)?;
    let n = r.count()?;
    let queries = (0..n).map(|_| read_query(&mut r)).collect::<Option<_>>()?;
    (r.remaining() == 0).then_some(StarkProof { trace_roots, comp_root, ood_frame, fri, queries })
}
