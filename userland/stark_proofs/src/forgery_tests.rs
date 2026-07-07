// NONOS Operating System (AGPL-3.0-or-later)
use crate::crypto::stark::air::{poseidon_trace, stark_prove, stark_verify, PoseidonPreimage};
use crate::crypto::stark::field::Fp;

extern crate alloc;
use alloc::vec::Vec;

// The STARK verifier makes two promises: it never accepts a forgery, and it
// never panics on any input, however malformed. It reads an attacker-supplied
// proof, so both must hold for every mutation of a valid proof and for wholly
// arbitrary proof structures. These fuzzes exercise both over adversarial
// input, the way the ZK verifier fuzz found real soundness bugs elsewhere in
// the tree.

const QUERIES: usize = 24;

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn honest() -> (PoseidonPreimage, crate::crypto::stark::air::StarkProof) {
    let input: [Fp; 8] = core::array::from_fn(|i| Fp::from_u64(7 * i as u64 + 3));
    let (trace, digest) = poseidon_trace(input);
    let air = PoseidonPreimage { digest };
    let proof = stark_prove(&air, &trace, QUERIES);
    (air, proof)
}

// The honest proof must verify, so the rejections below are meaningful.
#[test]
fn the_reference_proof_verifies() {
    let (air, proof) = honest();
    assert!(stark_verify(&air, &proof, QUERIES), "the reference proof was rejected");
}

// Every single-field mutation of a valid proof must be rejected, and the
// verifier must return rather than panic on each. The mutation space covers
// each committed root byte, each opened field element, each Merkle path node,
// and the composition and FRI openings.
#[test]
fn every_single_field_mutation_is_rejected_without_panic() {
    let (air, base) = honest();
    let mut s = 0x9e3779b97f4a7c15u64;
    let mut checked = 0u64;

    for _ in 0..6_000 {
        let mut p = base.clone();
        match xorshift(&mut s) % 9 {
            0 => {
                // Flip a bit in a trace column commitment.
                let r = (xorshift(&mut s) as usize) % p.trace_roots.len();
                let b = (xorshift(&mut s) as usize) % 32;
                p.trace_roots[r][b] ^= 1 << (xorshift(&mut s) % 8);
            }
            1 => {
                // Perturb a committed composition value at a query.
                if p.queries.is_empty() {
                    continue;
                }
                let q = (xorshift(&mut s) as usize) % p.queries.len();
                p.queries[q].comp = p.queries[q].comp + Fp::ONE;
            }
            2 => {
                // Perturb a trace window value at a query.
                let q = (xorshift(&mut s) as usize) % p.queries.len();
                if p.queries[q].window.is_empty() {
                    continue;
                }
                let w = (xorshift(&mut s) as usize) % p.queries[q].window.len();
                p.queries[q].window[w] = p.queries[q].window[w] + Fp::ONE;
            }
            3 => {
                // Corrupt a Merkle path node on a window opening.
                let q = (xorshift(&mut s) as usize) % p.queries.len();
                if p.queries[q].window_paths.is_empty() {
                    continue;
                }
                let wp = (xorshift(&mut s) as usize) % p.queries[q].window_paths.len();
                if p.queries[q].window_paths[wp].is_empty() {
                    continue;
                }
                let node = (xorshift(&mut s) as usize) % p.queries[q].window_paths[wp].len();
                let b = (xorshift(&mut s) as usize) % 32;
                p.queries[q].window_paths[wp][node][b] ^= 1;
            }
            4 => {
                // Corrupt the composition Merkle path.
                let q = (xorshift(&mut s) as usize) % p.queries.len();
                if p.queries[q].comp_path.is_empty() {
                    continue;
                }
                let node = (xorshift(&mut s) as usize) % p.queries[q].comp_path.len();
                let b = (xorshift(&mut s) as usize) % 32;
                p.queries[q].comp_path[node][b] ^= 1;
            }
            5 => {
                // Flip a bit in a FRI layer root.
                if p.fri.roots.is_empty() {
                    continue;
                }
                let r = (xorshift(&mut s) as usize) % p.fri.roots.len();
                let b = (xorshift(&mut s) as usize) % 32;
                p.fri.roots[r][b] ^= 1;
            }
            6 => {
                // Perturb a FRI final-layer coefficient.
                if p.fri.final_layer.is_empty() {
                    continue;
                }
                let f = (xorshift(&mut s) as usize) % p.fri.final_layer.len();
                p.fri.final_layer[f] = p.fri.final_layer[f] + Fp::ONE;
            }
            7 => {
                // Perturb a FRI layer opening value.
                if p.fri.queries.is_empty() {
                    continue;
                }
                let q = (xorshift(&mut s) as usize) % p.fri.queries.len();
                if p.fri.queries[q].layers.is_empty() {
                    continue;
                }
                let l = (xorshift(&mut s) as usize) % p.fri.queries[q].layers.len();
                p.fri.queries[q].layers[l].a = p.fri.queries[q].layers[l].a + Fp::ONE;
            }
            _ => {
                // Perturb a FRI layer sibling value.
                if p.fri.queries.is_empty() {
                    continue;
                }
                let q = (xorshift(&mut s) as usize) % p.fri.queries.len();
                if p.fri.queries[q].layers.is_empty() {
                    continue;
                }
                let l = (xorshift(&mut s) as usize) % p.fri.queries[q].layers.len();
                p.fri.queries[q].layers[l].b = p.fri.queries[q].layers[l].b + Fp::ONE;
            }
        }
        assert!(!stark_verify(&air, &p, QUERIES), "a mutated proof verified");
        checked += 1;
    }
    assert!(checked > 4_000, "too few mutations exercised the verifier");
}

// Structurally malformed proofs, with truncated, extended, or reordered
// vectors, must be rejected without panic. These exercise the verifier's
// length and bounds guards, not just its algebra.
#[test]
fn structural_mutations_never_panic_and_never_verify() {
    let (air, base) = honest();
    let mut s = 0x1234_5678_9abc_def0u64;

    for _ in 0..4_000 {
        let mut p = base.clone();
        match xorshift(&mut s) % 8 {
            0 => {
                p.trace_roots.pop();
            }
            1 => {
                p.trace_roots.push([0u8; 32]);
            }
            2 => {
                p.queries.pop();
            }
            3 => {
                if let Some(q) = p.queries.first().cloned() {
                    p.queries.push(q);
                }
            }
            4 => {
                if let Some(q) = p.queries.first_mut() {
                    q.window.pop();
                }
            }
            5 => {
                if let Some(q) = p.queries.first_mut() {
                    q.window_paths.pop();
                }
            }
            6 => {
                p.fri.roots.pop();
            }
            _ => {
                p.fri.final_layer.clear();
            }
        }
        // No assertion on the value beyond not panicking, except that an
        // accept would be a soundness break; a malformed proof must never
        // verify.
        assert!(!stark_verify(&air, &p, QUERIES), "a malformed proof verified");
    }
}

// A proof assembled from arbitrary bytes must be rejected and must not panic.
#[test]
fn arbitrary_proofs_are_rejected_without_panic() {
    use crate::crypto::stark::air::{StarkProof, StarkQuery};
    use crate::crypto::stark::fri::{FriProof, LayerOpening, QueryProof};

    let (air, _) = honest();
    let mut s = 0xdead_beef_cafe_babeu64;

    for _ in 0..2_000 {
        let n_roots = (xorshift(&mut s) % 6) as usize;
        let trace_roots: Vec<[u8; 32]> = (0..n_roots)
            .map(|_| {
                let mut r = [0u8; 32];
                for b in r.iter_mut() {
                    *b = (xorshift(&mut s) & 0xff) as u8;
                }
                r
            })
            .collect();

        let n_fri_roots = (xorshift(&mut s) % 6) as usize;
        let fri = FriProof {
            roots: (0..n_fri_roots).map(|_| [0u8; 32]).collect(),
            final_layer: (0..(xorshift(&mut s) % 8) as usize)
                .map(|_| Fp::from_u64(xorshift(&mut s)))
                .collect(),
            queries: (0..(xorshift(&mut s) % 6) as usize)
                .map(|_| QueryProof {
                    layers: (0..(xorshift(&mut s) % 6) as usize)
                        .map(|_| LayerOpening {
                            a: Fp::from_u64(xorshift(&mut s)),
                            a_path: Vec::new(),
                            b: Fp::from_u64(xorshift(&mut s)),
                            b_path: Vec::new(),
                        })
                        .collect(),
                })
                .collect(),
        };

        let queries: Vec<StarkQuery> = (0..(xorshift(&mut s) % 6) as usize)
            .map(|_| StarkQuery {
                comp: Fp::from_u64(xorshift(&mut s)),
                comp_path: Vec::new(),
                window: (0..(xorshift(&mut s) % 30) as usize)
                    .map(|_| Fp::from_u64(xorshift(&mut s)))
                    .collect(),
                window_paths: Vec::new(),
            })
            .collect();

        let p = StarkProof { trace_roots, fri, queries };
        assert!(!stark_verify(&air, &p, QUERIES), "an arbitrary proof verified");
    }
}
