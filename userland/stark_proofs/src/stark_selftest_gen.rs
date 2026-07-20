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

// Emit a REAL serialized money-grade STARK proof and its parameters, so the
// Solidity verifier author has an exact byte layout + a self-test vector to parse.
// The layout is documented in spec/stark-serialization.md and produced here from
// the live prover. Run explicitly (it writes a file):
//   cargo test gen_stark_selftest -- --ignored --nocapture

use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Accumulator, StarkProofExt};
use crate::crypto::stark::field::{Fp, Fp2};

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

fn u32le(b: &mut Vec<u8>, x: u32) {
    b.extend_from_slice(&x.to_le_bytes());
}
fn u64le(b: &mut Vec<u8>, x: u64) {
    b.extend_from_slice(&x.to_le_bytes());
}
fn fp(b: &mut Vec<u8>, x: Fp) {
    u64le(b, x.value());
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

/// Serialize a money-grade proof to the frozen byte layout (spec/stark-serialization.md).
pub(crate) fn serialize(proof: &StarkProofExt) -> Vec<u8> {
    let mut b = Vec::new();
    // trace_root (single wide-leaf commitment over the trace rows)
    b.extend_from_slice(&proof.trace_root);
    // comp_root
    b.extend_from_slice(&proof.comp_root);
    // ood_frame
    u32le(&mut b, proof.ood_frame.len() as u32);
    for v in &proof.ood_frame {
        fp2(&mut b, *v);
    }
    // fri: roots
    u32le(&mut b, proof.fri.roots.len() as u32);
    for r in &proof.fri.roots {
        b.extend_from_slice(r);
    }
    // fri: final_layer
    u32le(&mut b, proof.fri.final_layer.len() as u32);
    for v in &proof.fri.final_layer {
        fp2(&mut b, *v);
    }
    // fri: queries
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
    // fri: pow_nonce
    u64le(&mut b, proof.fri.pow_nonce);
    // consistency queries
    u32le(&mut b, proof.queries.len() as u32);
    for q in &proof.queries {
        fp2(&mut b, q.deep);
        path(&mut b, &q.deep_path);
        u32le(&mut b, q.trace.len() as u32);
        for t in &q.trace {
            fp(&mut b, *t);
        }
        // single wide-leaf path authenticating the whole trace row
        path(&mut b, &q.trace_path);
        fp2(&mut b, q.comp);
        path(&mut b, &q.comp_path);
    }
    b
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &x in bytes {
        s.push(char::from_digit((x >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((x & 0xf) as u32, 16).unwrap());
    }
    s
}

fn neg(x: u64) -> Fp {
    Fp::ZERO - Fp::from_u64(x)
}

#[test]
#[ignore]
fn gen_stark_selftest() {
    // The engine-level self-test proof: the value-conservation AIR (addends cancel).
    // This is the engine vector, NOT the full join-split; the deploy self-test must
    // swap to a full join-split proof before the pool verifier goes immutable.
    let air = Accumulator { log_t: 3 };
    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let mut trace = Vec::with_capacity(addends.len() * 2);
    let mut acc = Fp::ZERO;
    for &a in &addends {
        trace.push(acc);
        trace.push(a);
        acc = acc + a;
    }

    let n_queries = 32usize;
    let grind_bits = 8u32;
    let proof = stark_prove_ext(&air, &trace, n_queries, grind_bits);
    assert!(
        stark_verify_ext(&air, &proof, n_queries, grind_bits),
        "self-test proof does not verify"
    );

    let bytes = serialize(&proof);
    let json = alloc::format!(
        "{{\n  \"engine\": \"nonos-money-grade-stark\",\n  \"air\": \"accumulator-conservation\",\n  \"warning\": \"ENGINE-LEVEL vector only. NOT the full join-split. The pool verifier must NOT go immutable against this; swap to a full join-split proof first.\",\n  \"params\": {{ \"log_t\": 3, \"trace_width\": 2, \"n_queries\": {}, \"grind_bits\": {}, \"log_blowup\": 1, \"n_folds\": 3 }},\n  \"field\": \"goldilocks p=2^64-2^32+1\",\n  \"extension\": \"Fp2 = X^2 - 7\",\n  \"public\": {{ \"boundaries\": [[0, 0, \"0\"], [0, 7, \"0\"]] }},\n  \"proof_len_bytes\": {},\n  \"proof_hex\": \"{}\"\n}}\n",
        n_queries,
        grind_bits,
        bytes.len(),
        hex(&bytes)
    );

    let p = "/Users/ek/Desktop/NOX-SmartContract/spec/shield-selftest.json";
    std::fs::write(p, &json).expect("write self-test");
    std::println!("wrote {} proof bytes ({} json) to {}", bytes.len(), json.len(), p);
}

#[test]
#[ignore]
fn gen_join_split_selftest() {
    // A WIRED multi-region proof: conservation + range, with the input value bound
    // to the range-checked value by a copy constraint. This is the join-split shape
    // (WiredExt compose_ext + grand product + two constraint kinds); the full
    // membership/nullifier/commitment circuit adds Poseidon regions the same way.
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, Accumulator, AirExt, RangeCheck, WiredExt,
    };
    use alloc::boxed::Box;

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sigma: Vec<usize> = (0..32).collect();
    sigma.swap(1, 8); // conservation acc[1] (=input 7) wired to range acc[0]
    let wired = WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));

    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let mut cons = Vec::with_capacity(addends.len() * 2);
    let mut acc = Fp::ZERO;
    for &a in &addends {
        cons.push(acc);
        cons.push(a);
        acc = acc + a;
    }
    let mut rng = Vec::with_capacity(32);
    let mut v = 7u64;
    for i in 0..16usize {
        let bit = if i < 15 { v & 1 } else { 0 };
        rng.push(Fp::from_u64(v));
        rng.push(Fp::from_u64(bit));
        if i < 15 {
            v >>= 1;
        }
    }
    let witness = wired.trace(&[cons, rng]);
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(stark_verify_ext(&wired, &proof, 32, 8), "wired join-split self-test does not verify");

    let bytes = serialize(&proof);
    let json = alloc::format!(
        "{{\n  \"engine\": \"nonos-money-grade-stark\",\n  \"air\": \"wired-join-split (conservation + range, value bound)\",\n  \"warning\": \"WIRED MULTI-REGION vector: conservation + range + copy-constraint binding. Still NOT the full join-split (adds Poseidon membership/nullifier/commitment regions). The pool verifier must NOT go immutable against this.\",\n  \"params\": {{ \"n_queries\": 32, \"grind_bits\": 8 }},\n  \"regions\": [\"accumulator(log_t=3)\", \"range_check(log_t=4)\"],\n  \"wired_cols\": [0],\n  \"proof_len_bytes\": {},\n  \"proof_hex\": \"{}\"\n}}\n",
        bytes.len(),
        hex(&bytes)
    );
    let p = "/Users/ek/Desktop/NOX-SmartContract/spec/join-split-selftest.json";
    std::fs::write(p, &json).expect("write join-split self-test");
    std::println!("wrote {} proof bytes to {}", bytes.len(), p);
}
