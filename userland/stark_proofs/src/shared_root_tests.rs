// NONOS Operating System (AGPL-3.0-or-later)
//! Enrolling a capsule set proves many openings against one policy root. Pinning
//! that root per opening makes the constraint set grow with the batch, so the
//! shared-root form enforces it with one periodic checkpoint instead. These check
//! it proves the same statement: the real root verifies, and a root the prover
//! did not open against does not.

use crate::crypto::stark::air::{
    measure_capsule, stark_prove_ext_blown_bound, stark_verify_ext_blown_bound, MultiMembership,
    Opening, Poseidon, RATE,
};
use crate::crypto::stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::poseidon_merkle::PoseidonMerkleTree;
use alloc::vec::Vec;

const LEAVES: usize = 8;
const BATCH: usize = 4;

fn hasher() -> Poseidon {
    Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE])
}

fn ctx() -> Vec<u8> {
    (0..48u8).collect()
}

fn leaves(h: &Poseidon, salt: u8) -> Vec<[Fp; RATE]> {
    (0..LEAVES)
        .map(|k| {
            let img: Vec<u8> =
                (0..512usize).map(|i| (i * 31 + k * 7 + salt as usize + 1) as u8).collect();
            measure_capsule(h, &img)
        })
        .collect()
}

fn openings(tree: &PoseidonMerkleTree, ls: &[[Fp; RATE]]) -> Vec<Opening> {
    (0..BATCH)
        .map(|i| {
            let siblings = tree.open(i);
            let depth = siblings.len();
            Opening {
                leaf: ls[i],
                root: tree.root(),
                siblings,
                directions: (0..depth).map(|k| (i >> k) & 1 == 1).collect(),
            }
        })
        .collect()
}

fn prove(air: &MultiMembership) -> Vec<u8> {
    let trace = air.trace();
    let proof =
        stark_prove_ext_blown_bound(air, &trace, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS, &ctx());
    crate::crypto::stark::air::serialize_proof_ext(&proof)
}

fn verifies(air: &MultiMembership, bytes: &[u8]) -> bool {
    match crate::crypto::stark::air::deserialize_proof_ext(bytes) {
        Some(p) => {
            stark_verify_ext_blown_bound(air, &p, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS, &ctx())
        }
        None => false,
    }
}

#[test]
fn the_shared_root_form_verifies_the_whole_batch() {
    let h = hasher();
    let ls = leaves(&h, 0);
    let tree = PoseidonMerkleTree::commit(&h, &ls);
    let air =
        MultiMembership::new_shared_root(h.clone(), LOG_ROUNDS, tree.root(), openings(&tree, &ls));
    let proof = prove(&air);
    assert!(verifies(&air, &proof), "an honest batch did not verify");
}

#[test]
fn a_different_root_is_rejected() {
    let h = hasher();
    let ls = leaves(&h, 0);
    let tree = PoseidonMerkleTree::commit(&h, &ls);
    let air =
        MultiMembership::new_shared_root(h.clone(), LOG_ROUNDS, tree.root(), openings(&tree, &ls));
    let proof = prove(&air);

    let mut wrong = tree.root();
    wrong[0] = wrong[0] + Fp::from_u64(1);
    let bad = MultiMembership::new_shared_root(h, LOG_ROUNDS, wrong, openings(&tree, &ls));
    assert!(!verifies(&bad, &proof), "a proof verified against a root it never opened");
}

#[test]
fn a_proof_for_another_tree_is_rejected_under_the_real_root() {
    let h = hasher();
    let real = leaves(&h, 0);
    let real_tree = PoseidonMerkleTree::commit(&h, &real);

    let other = leaves(&h, 99);
    let other_tree = PoseidonMerkleTree::commit(&h, &other);
    assert_ne!(real_tree.root(), other_tree.root(), "the two trees must differ");

    let forged = MultiMembership::new_shared_root(
        h.clone(),
        LOG_ROUNDS,
        other_tree.root(),
        openings(&other_tree, &other),
    );
    let forged_proof = prove(&forged);

    let real_air = MultiMembership::new_shared_root(
        h,
        LOG_ROUNDS,
        real_tree.root(),
        openings(&real_tree, &real),
    );
    assert!(
        !verifies(&real_air, &forged_proof),
        "a proof about a tree the prover invented passed under the real policy root"
    );
}

#[test]
fn a_capsule_outside_the_tree_cannot_be_opened() {
    let h = hasher();
    let ls = leaves(&h, 0);
    let tree = PoseidonMerkleTree::commit(&h, &ls);

    // A leaf the tree never committed, kept at the real root and path.
    let mut ops = openings(&tree, &ls);
    ops[0].leaf[0] = ops[0].leaf[0] + Fp::from_u64(1);
    let air = MultiMembership::new_shared_root(h, LOG_ROUNDS, tree.root(), ops);
    let proof = prove(&air);
    assert!(!verifies(&air, &proof), "a leaf outside the committed tree folded to the policy root");
}

#[test]
fn only_the_public_root_is_enforced_not_the_openings_own() {
    // A prover that rewrites the root carried on each opening changes nothing:
    // the checkpoint enforces the root passed as the public statement.
    let h = hasher();
    let ls = leaves(&h, 0);
    let tree = PoseidonMerkleTree::commit(&h, &ls);

    let mut ops = openings(&tree, &ls);
    for o in ops.iter_mut() {
        o.root[0] = o.root[0] + Fp::from_u64(1);
    }
    let air = MultiMembership::new_shared_root(h.clone(), LOG_ROUNDS, tree.root(), ops);
    let proof = prove(&air);

    let honest = MultiMembership::new_shared_root(h, LOG_ROUNDS, tree.root(), openings(&tree, &ls));
    assert!(verifies(&honest, &proof), "the public root should be what is enforced");
}
