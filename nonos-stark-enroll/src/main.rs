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

//! Enrollment tool. Measures capsule and kernel images, commits them to one
//! policy tree, and emits each image's transparent STARK attestation trailer
//! bound to its identity. The tree is padded to the gate's fixed depth with a
//! domain-separated slot that no ELF can occupy, the root is deterministic, and
//! every emitted trailer is re-checked here with the exact parse the kernel
//! spawn gate runs, so a trailer that would not verify at boot fails at build.

use std::env;
use std::fs;
use std::process::exit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use nonos_stark::air::{
    build_attestation_trailer_from_set, deserialize_proof_ext, stark_verify_ext_blown_bound,
    MeasuredSet, MerkleMembership, Poseidon, RATE,
};
use nonos_stark::field::Fp;
// One definition, in nonos_stark. Prover and verifier must
// agree exactly; a drift downward in queries or grinding still verifies.
use nonos_stark::attest_params::{
    EXTRA_BLOWUP_BITS as EXTRA_BLOWUP, GRIND_BITS, LOG_ROUNDS, N_QUERIES,
};

const POLICY_EPOCH: u64 = 1;
const BOOT_EPOCH: u64 = 1;
const POLICY_TREE_DEPTH: usize = 8;
const LEAVES: usize = 1 << POLICY_TREE_DEPTH;
const MAGIC: &[u8; 8] = b"NZKSTRK1";

/// The padding image for unused policy slots. It begins with a byte no ELF
/// starts with, so a real capsule can never measure to a padding leaf.
const PAD_IMAGE: &[u8] = b"\x00NONOS-POLICY-RESERVED-SLOT-v1";

/// A capsule context: its measurement, its granted capabilities, the epoch.
fn capsule_context(image: &[u8], granted_caps: u64) -> Vec<u8> {
    let mut ctx = Vec::with_capacity(48);
    ctx.extend_from_slice(blake3::hash(image).as_bytes());
    ctx.extend_from_slice(&granted_caps.to_be_bytes());
    ctx.extend_from_slice(&POLICY_EPOCH.to_be_bytes());
    ctx
}

/// The kernel self-attestation context: its measurement and the boot epoch.
fn kernel_context(image: &[u8]) -> Vec<u8> {
    let mut ctx = Vec::with_capacity(40);
    ctx.extend_from_slice(blake3::hash(image).as_bytes());
    ctx.extend_from_slice(&BOOT_EPOCH.to_be_bytes());
    ctx
}

/// Four little-endian words into a rate-width digest, as the gate reads a root.
fn to_rate(bytes: &[u8]) -> [Fp; RATE] {
    let mut out = [Fp::ZERO; RATE];
    for (i, lane) in out.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&bytes[i * 8..i * 8 + 8]);
        *lane = Fp::from_u64(u64::from_le_bytes(w));
    }
    out
}

/// A rate-width root serialized as the gate expects to read it back.
fn root_to_bytes(root: [Fp; RATE]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, lane) in root.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    out
}

/// Pad a set of real images to the fixed policy-tree width with reserved slots.
fn padded_images<'a>(images: &[&'a [u8]]) -> Vec<&'a [u8]> {
    assert!(images.len() <= LEAVES, "more images than policy slots");
    let mut v: Vec<&[u8]> = images.to_vec();
    while v.len() < LEAVES {
        v.push(PAD_IMAGE);
    }
    v
}

/// The kernel spawn gate's exact parse and verify, run here so an emitted
/// trailer that would be refused at boot is caught now. Returns the verdict.
fn gate_verify(root_bytes: &[u8; 32], trailer: &[u8], context: &[u8]) -> bool {
    let depth = POLICY_TREE_DEPTH;
    let dir_bytes = depth.div_ceil(8);
    let sib_end = 9 + depth * 32;
    if trailer.len() < sib_end + dir_bytes
        || &trailer[0..8] != MAGIC
        || trailer[8] as usize != depth
    {
        return false;
    }
    let mut siblings = Vec::with_capacity(depth);
    for i in 0..depth {
        siblings.push(to_rate(&trailer[9 + i * 32..9 + i * 32 + 32]));
    }
    let dirs = &trailer[sib_end..sib_end + dir_bytes];
    let directions: Vec<bool> = (0..depth).map(|i| (dirs[i / 8] >> (i % 8)) & 1 == 1).collect();
    let Some(proof) = deserialize_proof_ext(&trailer[sib_end + dir_bytes..]) else {
        return false;
    };
    let root = to_rate(root_bytes);
    let air = MerkleMembership::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        root,
        siblings,
        directions,
    );
    stark_verify_ext_blown_bound(&air, &proof, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP, context)
}

/// Enroll `images` under one policy root and emit a trailer for each, bound to
/// the matching `contexts`. Every trailer is verified with the gate parse before
/// it is returned; a trailer that does not verify is a hard error.
fn enroll(images: &[&[u8]], contexts: &[Vec<u8>]) -> ([u8; 32], Vec<Vec<u8>>) {
    assert_eq!(images.len(), contexts.len(), "one context per image");
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let padded = padded_images(images);
    // Measure and commit once. Every trailer opens this same tree, so measuring
    // per capsule would hash the whole image set once per capsule.
    let set = MeasuredSet::commit(&hasher, &padded);
    let root = root_to_bytes(set.root());

    let n = contexts.len();
    let counter = AtomicUsize::new(0);
    let workers = thread::available_parallelism().map(|v| v.get()).unwrap_or(1).min(n).max(1);
    let collected: Vec<Vec<(usize, Vec<u8>)>> = thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                scope.spawn(|| {
                    let mut local = Vec::new();
                    loop {
                        let i = counter.fetch_add(1, Ordering::Relaxed);
                        if i >= n {
                            break;
                        }
                        let ctx = &contexts[i];
                        let trailer = build_attestation_trailer_from_set(
                            &hasher,
                            LOG_ROUNDS,
                            &set,
                            i,
                            ctx,
                            N_QUERIES,
                            GRIND_BITS,
                            EXTRA_BLOWUP,
                        );
                        if !gate_verify(&root, &trailer, ctx) {
                            eprintln!("enroll: trailer {i} failed the gate self-check");
                            exit(2);
                        }
                        local.push((i, trailer));
                    }
                    local
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut trailers: Vec<Vec<u8>> = vec![Vec::new(); n];
    for local in collected {
        for (i, trailer) in local {
            trailers[i] = trailer;
        }
    }
    (root, trailers)
}

/// Prove the enroll -> trailer -> gate loop end to end, including that a wrong
/// context and an unenrolled image are both refused.
fn selftest() {
    let cap_a = b"capsule:terminal image bytes".to_vec();
    let cap_b = b"capsule:net_core image bytes".to_vec();
    let kernel = b"kernel image bytes for self-attestation".to_vec();

    let images: [&[u8]; 3] = [&cap_a, &cap_b, &kernel];
    let contexts = vec![
        capsule_context(&cap_a, 0x0000_0000_0000_0007),
        capsule_context(&cap_b, 0x0000_0000_0000_0011),
        kernel_context(&kernel),
    ];
    let (root, trailers) = enroll(&images, &contexts);

    for (i, trailer) in trailers.iter().enumerate() {
        assert!(gate_verify(&root, trailer, &contexts[i]), "enrolled image {i} refused");
    }
    let wrong = capsule_context(&cap_a, 0x0000_0000_0000_00FF);
    assert!(!gate_verify(&root, &trailers[0], &wrong), "wrong capability context accepted");
    let rogue = capsule_context(b"capsule:rogue never enrolled", 0x7);
    assert!(!gate_verify(&root, &trailers[0], &rogue), "rogue measurement accepted");

    println!("selftest OK: {} images enrolled under root {}", images.len(), hex(&root));
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn read(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        exit(1)
    })
}

fn write(path: &str, bytes: &[u8]) {
    fs::write(path, bytes).unwrap_or_else(|e| {
        eprintln!("cannot write {path}: {e}");
        exit(1)
    });
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  \
         nonos-stark-enroll selftest\n  \
         nonos-stark-enroll kernel <kernel-image> <root.bin> <trailer.bin>\n  \
         nonos-stark-enroll capsules <root.bin> <CAPS:image:trailer-out> ..."
    );
    exit(1)
}

/// Enroll a whole capsule set under one policy root and write each capsule's
/// trailer. Each spec is `CAPS:image:trailer-out`, where CAPS is the granted
/// capability mask in hex. All capsules share the tree, so the root commits to
/// the set and each trailer proves that capsule under exactly its capabilities.
fn enroll_capsules(root_out: &str, specs: &[String]) {
    let mut images: Vec<Vec<u8>> = Vec::new();
    let mut contexts: Vec<Vec<u8>> = Vec::new();
    let mut outs: Vec<String> = Vec::new();
    for spec in specs {
        let parts: Vec<&str> = spec.splitn(3, ':').collect();
        if parts.len() != 3 {
            eprintln!("bad spec {spec}, want CAPS:image:trailer-out");
            exit(1);
        }
        let caps =
            u64::from_str_radix(parts[0].trim_start_matches("0x"), 16).unwrap_or_else(|_| {
                eprintln!("bad capability mask {}", parts[0]);
                exit(1)
            });
        let image = read(parts[1]);
        contexts.push(capsule_context(&image, caps));
        images.push(image);
        outs.push(parts[2].to_string());
    }
    let refs: Vec<&[u8]> = images.iter().map(Vec::as_slice).collect();
    let (root, trailers) = enroll(&refs, &contexts);
    write(root_out, &root);
    for (out, trailer) in outs.iter().zip(&trailers) {
        write(out, trailer);
    }
    println!("enrolled {} capsules under root {}", images.len(), hex(&root));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("selftest") => selftest(),
        Some("kernel") if args.len() == 5 => {
            let image = read(&args[2]);
            let ctx = kernel_context(&image);
            let (root, trailers) = enroll(&[&image], &[ctx]);
            write(&args[3], &root);
            write(&args[4], &trailers[0]);
            println!("kernel enrolled under root {}", hex(&root));
        }
        Some("capsules") if args.len() >= 4 => enroll_capsules(&args[2], &args[3..]),
        _ => usage(),
    }
}
