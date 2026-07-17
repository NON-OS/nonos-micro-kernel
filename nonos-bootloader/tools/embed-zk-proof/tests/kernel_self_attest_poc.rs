// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Proof of concept for the kernel self-attestation, end to end on the host.
//!
//! It exercises the exact chain the boot path uses: enroll the kernel bytes with
//! nonos-stark, embed the trailer into the image with the real footer assembler,
//! parse the footer exactly as the bootloader does, and verify the trailer with
//! the same verifier the bootloader links. A tampered kernel is rejected. This
//! stands in for the QEMU boot short of running one: the same code, the same
//! byte layout, the same verdict.

use embed_zk_proof::{assemble_attested_image, SignedKernel};
use nonos_stark::air::{
    build_attestation_trailer, enroll_policy_root, verify_membership_trailer, Poseidon, RATE,
};
use nonos_stark::field::Fp;

// The constants the bootloader's stark_attest.rs and the enrollment tool agree on.
const LOG_ROUNDS: u32 = 3;
const DEPTH: usize = 8;
const LEAVES: usize = 1 << DEPTH;
const N_QUERIES: usize = 32;
const GRIND_BITS: u32 = 16;
const EXTRA_BLOWUP_BITS: u32 = 3;
const BOOT_EPOCH: u64 = 1;
const PAD_IMAGE: &[u8] = b"\x00NONOS-POLICY-RESERVED-SLOT-v1";

/// The kernel self-attestation context: its measurement and the boot epoch. This
/// is byte-for-byte what `verify_kernel_self_attestation` builds in the bootloader.
fn kernel_context(kernel_bytes: &[u8]) -> Vec<u8> {
    let mut ctx = Vec::with_capacity(40);
    ctx.extend_from_slice(blake3::hash(kernel_bytes).as_bytes());
    ctx.extend_from_slice(&BOOT_EPOCH.to_be_bytes());
    ctx
}

/// A rate-width root serialized as the gate reads it back (four little-endian words).
fn root_to_bytes(root: [Fp; RATE]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, lane) in root.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    out
}

/// Enroll a kernel image: pad the tree to the gate depth, commit, and build the
/// trailer bound to the kernel context. Returns the serialized root and trailer.
fn enroll_kernel(kernel_bytes: &[u8]) -> ([u8; 32], Vec<u8>) {
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let mut images: Vec<&[u8]> = vec![kernel_bytes];
    while images.len() < LEAVES {
        images.push(PAD_IMAGE);
    }
    let root = root_to_bytes(enroll_policy_root(&hasher, &images));
    let ctx = kernel_context(kernel_bytes);
    let trailer = build_attestation_trailer(
        &hasher, LOG_ROUNDS, &images, 0, &ctx, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS,
    );
    (root, trailer)
}

/// The bootloader's footer parse: recover the kernel region and the proof region
/// from an assembled image. Mirrors image_format::parse in nonos-bootloader.
fn parse_footer(image: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let f = image.len() - embed_zk_proof::FOOTER_SIZE;
    let u32_at = |o: usize| {
        u32::from_le_bytes([image[f + o], image[f + o + 1], image[f + o + 2], image[f + o + 3]])
            as usize
    };
    let kernel_size = u32_at(28);
    let proof_offset = u32_at(40);
    let proof_size = u32_at(44);
    let kernel_bytes = image[0..kernel_size].to_vec();
    let proof_bytes = image[proof_offset..proof_offset + proof_size].to_vec();
    (kernel_bytes, proof_bytes)
}

/// Verify a trailer exactly as the bootloader does before the jump.
fn boot_verify(root: &[u8; 32], kernel_bytes: &[u8], trailer: &[u8]) -> bool {
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    verify_membership_trailer(
        &hasher,
        LOG_ROUNDS,
        *root,
        DEPTH,
        trailer,
        &kernel_context(kernel_bytes),
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    )
}

fn signed_kernel(kernel_bytes: &[u8]) -> SignedKernel {
    SignedKernel {
        raw_bytes: Vec::new(),
        kernel_bytes: kernel_bytes.to_vec(),
        signature: vec![0u8; 64],
        signature_algorithm: 1,
        rollback_index: 1,
    }
}

#[test]
fn the_kernel_self_attestation_survives_embed_and_boot_verify() {
    let kernel_bytes = b"nonos-kernel code region, the exact bytes the bootloader measures".to_vec();

    // Enroll and embed, the build side.
    let (root, trailer) = enroll_kernel(&kernel_bytes);
    let image = assemble_attested_image(&signed_kernel(&kernel_bytes), trailer.clone());

    // Parse the footer, the boot side.
    let (parsed_kernel, parsed_proof) = parse_footer(&image.data);
    assert_eq!(parsed_kernel, kernel_bytes, "the measured kernel region must be unchanged");
    assert_eq!(parsed_proof, trailer, "the STARK trailer must round-trip through the footer");

    // Verify, before the jump.
    assert!(
        boot_verify(&root, &parsed_kernel, &parsed_proof),
        "the kernel self-attestation must verify against the enrolled root"
    );
}

#[test]
fn a_tampered_kernel_fails_self_attestation() {
    let kernel_bytes = b"nonos-kernel code region, the exact bytes the bootloader measures".to_vec();
    let (root, trailer) = enroll_kernel(&kernel_bytes);

    let mut tampered = kernel_bytes.clone();
    tampered[0] ^= 0x01;
    assert!(
        !boot_verify(&root, &tampered, &trailer),
        "a kernel that does not match its enrolled measurement must be refused"
    );
}
