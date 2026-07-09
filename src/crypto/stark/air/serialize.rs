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

//! Canonical byte encoding of a STARK proof, so an attestation can travel in a
//! trailer. Length-prefixed and deterministic; `deserialize` is the inverse.

use super::super::field::Fp;
use super::types::StarkProof;
use alloc::vec::Vec;

fn put_len(out: &mut Vec<u8>, n: usize) {
    out.extend_from_slice(&(n as u32).to_le_bytes());
}

fn put_fp(out: &mut Vec<u8>, v: Fp) {
    out.extend_from_slice(&v.value().to_le_bytes());
}

fn put_digests(out: &mut Vec<u8>, ds: &[[u8; 32]]) {
    put_len(out, ds.len());
    for d in ds {
        out.extend_from_slice(d);
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
    out.extend_from_slice(&p.comp_root);
    put_fps(&mut out, &p.ood_frame);

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
