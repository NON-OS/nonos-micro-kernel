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

//! Canonical byte encoding of a STARK proof, so an attestation proof can travel
//! in a capsule trailer or a boot footer. The encoding is length-prefixed and
//! deterministic, and `deserialize_proof` reads attacker-controlled bytes: it is
//! total, never panics, allocates only what the input can back, and rejects a
//! non-canonical field element rather than reducing it, so the round trip is a
//! bijection. A malformed proof is rejected here or, if it decodes, by the
//! verifier; it can never crash the reader.

use super::super::field::{Fp, P};
use super::super::fri::{FriProof, LayerOpening, QueryProof};
use super::types::{StarkProof, StarkQuery};
use alloc::vec::Vec;

/// Append a length as a four-byte little-endian prefix.
fn put_len(out: &mut Vec<u8>, n: usize) {
    out.extend_from_slice(&(n as u32).to_le_bytes());
}

/// Append a field element as its eight canonical little-endian bytes.
fn put_fp(out: &mut Vec<u8>, v: Fp) {
    out.extend_from_slice(&v.value().to_le_bytes());
}

fn put_digest(out: &mut Vec<u8>, d: &[u8; 32]) {
    out.extend_from_slice(d);
}

fn put_digests(out: &mut Vec<u8>, ds: &[[u8; 32]]) {
    put_len(out, ds.len());
    for d in ds {
        put_digest(out, d);
    }
}

fn put_fps(out: &mut Vec<u8>, vs: &[Fp]) {
    put_len(out, vs.len());
    for v in vs {
        put_fp(out, *v);
    }
}

/// Encode a proof to its canonical bytes.
pub fn serialize_proof(p: &StarkProof) -> Vec<u8> {
    let mut out = Vec::new();
    put_digests(&mut out, &p.trace_roots);
    put_digest(&mut out, &p.comp_root);
    put_fps(&mut out, &p.ood_frame);
    // FRI proof.
    put_digests(&mut out, &p.fri.roots);
    put_fps(&mut out, &p.fri.final_layer);
    put_len(&mut out, p.fri.queries.len());
    for q in &p.fri.queries {
        put_len(&mut out, q.layers.len());
        for l in &q.layers {
            put_fp(&mut out, l.a);
            put_digests(&mut out, &l.a_path);
            put_fp(&mut out, l.b);
            put_digests(&mut out, &l.b_path);
        }
    }
    // Consistency queries.
    put_len(&mut out, p.queries.len());
    for q in &p.queries {
        put_fp(&mut out, q.deep);
        put_digests(&mut out, &q.deep_path);
        put_fps(&mut out, &q.trace);
        put_len(&mut out, q.trace_paths.len());
        for path in &q.trace_paths {
            put_digests(&mut out, path);
        }
        put_fp(&mut out, q.comp);
        put_digests(&mut out, &q.comp_path);
    }
    out
}

/// A bounds-checked cursor over the input. Every read returns `None` past the
/// end, so a truncated or crafted buffer fails cleanly instead of panicking.
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
        if n > self.remaining() {
            return None;
        }
        let s = &self.bytes[self.pos..self.pos + n];
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
        // Reject a non-canonical element rather than reducing it, so the
        // encoding is one to one.
        if v >= P {
            return None;
        }
        Some(Fp::from_u64(v))
    }

    fn digest(&mut self) -> Option<[u8; 32]> {
        let b = self.take(32)?;
        let mut d = [0u8; 32];
        d.copy_from_slice(b);
        Some(d)
    }

    fn digests(&mut self) -> Option<Vec<[u8; 32]>> {
        let n = self.len()?;
        // A length claiming more digests than the buffer can hold is rejected
        // before any allocation, so a crafted prefix cannot exhaust memory.
        if n > self.remaining() / 32 {
            return None;
        }
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.digest()?);
        }
        Some(v)
    }

    fn fps(&mut self) -> Option<Vec<Fp>> {
        let n = self.len()?;
        if n > self.remaining() / 8 {
            return None;
        }
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.fp()?);
        }
        Some(v)
    }
}

/// Decode a proof from bytes, or `None` if the bytes are not a canonical proof.
/// Total over every input: it never panics and allocates only what the input
/// length can back.
pub fn deserialize_proof(bytes: &[u8]) -> Option<StarkProof> {
    let mut r = Reader::new(bytes);

    let trace_roots = r.digests()?;
    let comp_root = r.digest()?;
    let ood_frame = r.fps()?;

    let fri_roots = r.digests()?;
    let fri_final = r.fps()?;
    let n_fri_q = r.len()?;
    // Each query proof needs at least its own length word; cap by that.
    if n_fri_q > r.remaining() / 4 {
        return None;
    }
    let mut fri_queries = Vec::with_capacity(n_fri_q);
    for _ in 0..n_fri_q {
        let n_layers = r.len()?;
        if n_layers > r.remaining() / 4 {
            return None;
        }
        let mut layers = Vec::with_capacity(n_layers);
        for _ in 0..n_layers {
            let a = r.fp()?;
            let a_path = r.digests()?;
            let b = r.fp()?;
            let b_path = r.digests()?;
            layers.push(LayerOpening { a, a_path, b, b_path });
        }
        fri_queries.push(QueryProof { layers });
    }
    let fri = FriProof { roots: fri_roots, final_layer: fri_final, queries: fri_queries };

    let n_q = r.len()?;
    if n_q > r.remaining() / 4 {
        return None;
    }
    let mut queries = Vec::with_capacity(n_q);
    for _ in 0..n_q {
        let deep = r.fp()?;
        let deep_path = r.digests()?;
        let trace = r.fps()?;
        let n_tp = r.len()?;
        if n_tp > r.remaining() / 4 {
            return None;
        }
        let mut trace_paths = Vec::with_capacity(n_tp);
        for _ in 0..n_tp {
            trace_paths.push(r.digests()?);
        }
        let comp = r.fp()?;
        let comp_path = r.digests()?;
        queries.push(StarkQuery { deep, deep_path, trace, trace_paths, comp, comp_path });
    }

    // A canonical proof consumes the whole buffer, with no trailing bytes.
    if r.remaining() != 0 {
        return None;
    }
    Some(StarkProof { trace_roots, comp_root, ood_frame, fri, queries })
}
