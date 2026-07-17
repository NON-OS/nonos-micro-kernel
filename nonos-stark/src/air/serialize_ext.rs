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

//! Serializing a money-grade proof to the frozen byte layout a capsule ships in its
//! attestation trailer. Little-endian throughout, matching the base serializer and
//! the deserializer that reads it back in the kernel gate.

use super::super::field::{Fp, Fp2};
use super::types_ext::StarkProofExt;
use alloc::vec::Vec;

/// The proof as bytes: roots, out-of-domain frame, the FRI layers with their
/// authentication paths, and each consistency query with its wide-leaf trace path.
pub fn serialize_proof_ext(proof: &StarkProofExt) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&proof.trace_root);
    b.extend_from_slice(&proof.comp_root);
    u32le(&mut b, proof.ood_frame.len() as u32);
    for v in &proof.ood_frame {
        fp2(&mut b, *v);
    }
    u32le(&mut b, proof.fri.roots.len() as u32);
    for r in &proof.fri.roots {
        b.extend_from_slice(r);
    }
    u32le(&mut b, proof.fri.final_layer.len() as u32);
    for v in &proof.fri.final_layer {
        fp2(&mut b, *v);
    }
    u32le(&mut b, proof.fri.queries.len() as u32);
    for q in &proof.fri.queries {
        u32le(&mut b, q.layers.len() as u32);
        for l in &q.layers {
            fp2(&mut b, l.a);
            path(&mut b, &l.a_path);
            fp2(&mut b, l.b);
            path(&mut b, &l.b_path);
        }
    }
    b.extend_from_slice(&proof.fri.pow_nonce.to_le_bytes());
    u32le(&mut b, proof.queries.len() as u32);
    for q in &proof.queries {
        fp2(&mut b, q.deep);
        path(&mut b, &q.deep_path);
        u32le(&mut b, q.trace.len() as u32);
        for t in &q.trace {
            fp(&mut b, *t);
        }
        path(&mut b, &q.trace_path);
        fp2(&mut b, q.comp);
        path(&mut b, &q.comp_path);
    }
    b
}

fn u32le(b: &mut Vec<u8>, x: u32) {
    b.extend_from_slice(&x.to_le_bytes());
}
fn fp(b: &mut Vec<u8>, x: Fp) {
    b.extend_from_slice(&x.value().to_le_bytes());
}
fn fp2(b: &mut Vec<u8>, x: Fp2) {
    fp(b, x.c0);
    fp(b, x.c1);
}
fn path(b: &mut Vec<u8>, p: &[[u8; 32]]) {
    u32le(b, p.len() as u32);
    for d in p {
        b.extend_from_slice(d);
    }
}
