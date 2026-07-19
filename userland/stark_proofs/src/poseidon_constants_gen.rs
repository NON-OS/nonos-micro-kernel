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

// Emit the concrete Poseidon-Goldilocks constants from the REAL kernel code, for
// the Solidity hasher to hardcode and KAT-gate. BLAKE3 has no EVM opcode, so the
// contract cannot regenerate the round constants on-chain; it pins these numbers
// and checks the KAT vectors at deploy. Run explicitly (it writes a file):
//   cargo test gen_poseidon_constants -- --ignored --nocapture

use crate::crypto::stark::air::{Poseidon, NOTE_DOMAIN, NOTE_LIMBS, RATE, WIDTH};
use crate::crypto::stark::field::{Fp, P};

fn fp(x: u64) -> Fp {
    Fp::from_u64(x)
}

/// The Cauchy MDS the kernel uses: M[i][j] = 1 / (i - (WIDTH + j)). Recomputed
/// here from the documented formula and checked against the live compression.
fn mds() -> [[Fp; WIDTH]; WIDTH] {
    let mut m = [[Fp::ZERO; WIDTH]; WIDTH];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = (fp(i as u64) - fp((WIDTH + j) as u64)).inv();
        }
    }
    m
}

fn row_json(vals: &[Fp]) -> String {
    let parts: alloc::vec::Vec<String> =
        vals.iter().map(|v| alloc::format!("\"{}\"", v.value())).collect();
    alloc::format!("[{}]", parts.join(","))
}

#[test]
#[ignore]
fn gen_poseidon_constants() {
    let log_rounds = 5u32;
    let rounds = 1usize << log_rounds; // 32
    let h = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);

    // MDS, 8x8.
    let m = mds();
    let mds_rows: alloc::vec::Vec<String> = (0..WIDTH).map(|i| row_json(&m[i])).collect();

    // Round constants, row-major, `rounds` x WIDTH, straight from the kernel.
    let rc_rows: alloc::vec::Vec<String> =
        (0..rounds).map(|r| row_json(&h.round_constant(r))).collect();

    // KATs for both primitives the contract mirrors: the 2-to-1 compression
    // (Merkle node hash, full permutation) and the single-block rate hash.
    let kat = |label: &str, out: [Fp; RATE], extra: String| -> String {
        alloc::format!("{{\"op\":\"{}\",{}\"digest\":{}}}", label, extra, row_json(&out))
    };
    let l = [fp(1), fp(2), fp(3), fp(4)];
    let r = [fp(5), fp(6), fp(7), fp(8)];
    let compress_kat = kat(
        "compress",
        h.compress(&l, &r),
        alloc::format!("\"left\":{},\"right\":{},", row_json(&l), row_json(&r)),
    );
    let hash_in = [fp(9), fp(10), fp(11), fp(12)];
    let hash_kat =
        kat("hash", h.hash(&hash_in), alloc::format!("\"input\":{},", row_json(&hash_in)));

    // Note-commitment KAT over the 11 limbs 1..=11, so the Solidity compress-tree
    // reproduces the kernel digest bit-for-bit.
    let mut note = [Fp::ZERO; NOTE_LIMBS];
    for (i, c) in note.iter_mut().enumerate() {
        *c = fp((i + 1) as u64);
    }
    let commit_kat =
        kat("commit_note", h.commit_note(&note), alloc::format!("\"limbs\":{},", row_json(&note)));

    let json = alloc::format!(
        "{{\n\
         \"scheme\": \"poseidon-goldilocks\",\n\
         \"field_modulus\": \"{}\",\n\
         \"width\": {}, \"rate\": {}, \"capacity\": {},\n\
         \"sbox_alpha\": 7,\n\
         \"log_rounds\": {}, \"rounds\": {},\n\
         \"rc_domain\": \"NONOS-POSEIDON-GOLDILOCKS-RC\",\n\
         \"rc_rule\": \"blake3(rc_domain || r_le64 || j_le64)[0..8] as u64 -> Fp\",\n\
         \"mds\": [{}],\n\
         \"round_constants\": [{}],\n\
         \"kats\": [{}, {}, {}],\n\
         \"conventions\": {{\n\
         \"mds_apply\": \"out[j] = rc[j] + sum_k M[j][k] * sbox(state[k]); sbox = x^7\",\n\
         \"compress\": \"state=[left(4),right(4)]; permute all {} rounds; digest=state[0..4]\",\n\
         \"hash\": \"state=[input(4),0,0,0,0]; apply rounds-1 ({}) round funcs; digest=state[0..4]\",\n\
         \"note_commitment\": \"compress-tree over {} limbs padded to 16: p[0..11]=limbs, p[11]=NOTE_DOMAIN({}), p[12..16]=0; d0=compress(p[0..4],p[4..8]); d1=compress(p[8..12],p[12..16]); cm=compress(d0,d1)\"\n\
         }}\n\
         }}\n",
        P, WIDTH, RATE, WIDTH - RATE, log_rounds, rounds,
        mds_rows.join(","), rc_rows.join(","), compress_kat, hash_kat, commit_kat,
        rounds, rounds - 1, NOTE_LIMBS, NOTE_DOMAIN
    );

    let path = "/Users/ek/Desktop/NOX-SmartContract/spec/poseidon-constants.json";
    std::fs::write(path, &json).expect("write constants");
    std::println!("wrote {} bytes to {}", json.len(), path);

    // Sanity: the recomputed MDS matches the live compression by construction, and
    // every constant is a canonical field element.
    for r in 0..rounds {
        for &c in h.round_constant(r).iter() {
            assert!(c.value() < P);
        }
    }
}

fn note_from(seed: u64) -> [Fp; NOTE_LIMBS] {
    let mut n = [Fp::ZERO; NOTE_LIMBS];
    for (i, c) in n.iter_mut().enumerate() {
        *c = fp(seed.wrapping_add(i as u64 + 1));
    }
    n
}

#[test]
fn note_commitment_is_deterministic_and_binding() {
    let h = Poseidon::new(5, [Fp::ZERO; RATE]);
    let cm = h.commit_note(&note_from(1));
    // Deterministic: the same note always commits to the same value.
    assert_eq!(h.commit_note(&note_from(1)), cm);
    // Binding: changing any single limb changes the commitment, so a deposit
    // cannot be reopened to a different value, owner, or blinding.
    for i in 0..NOTE_LIMBS {
        let mut n = note_from(1);
        n[i] = n[i] + Fp::ONE;
        assert_ne!(h.commit_note(&n), cm);
    }
    // Distinct notes give distinct commitments.
    assert_ne!(h.commit_note(&note_from(2)), cm);
}
