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

//! The categorized attack battery. Each entry builds a real attested image,
//! mounts one attack, runs the boot-side parse and verify, and records whether
//! the gate refused it. An attack "passes" only when it is refused.

use super::finding::{Finding, Severity};
use crate::attest::{assemble_image, enroll_kernel, parse_image_footer, verify_kernel_attestation};

/// Run the full attack battery against the kernel self-attestation.
pub fn battery() -> Vec<Finding> {
    let genuine = b"nonos-kernel code region, the exact bytes the bootloader measures".to_vec();
    let (root, trailer) = enroll_kernel(&genuine);
    let image = assemble_image(&genuine, trailer.clone());
    let mut out = Vec::new();

    // Sanity: the genuine image must verify, or every refusal below is vacuous.
    let (k, t) = parse_image_footer(&image).expect("genuine image must parse");
    out.push(Finding {
        id: "BASELINE",
        category: "sanity",
        severity: Severity::Critical,
        description: "the genuine image verifies (refusal here would be vacuous)",
        refused: verify_kernel_attestation(&root, &k, &t),
    });

    // integrity: the flashed image's kernel region is edited, trailer kept.
    let mut tampered = image.clone();
    tampered[10] ^= 0xff;
    let (k, t) = parse_image_footer(&tampered).expect("tampered image parses");
    out.push(Finding {
        id: "KRN-TAMPER",
        category: "integrity",
        severity: Severity::Critical,
        description: "flip a byte in the flashed kernel, keep the trailer",
        refused: !verify_kernel_attestation(&root, &k, &t),
    });

    // integrity: drop a byte, shifting the whole layout under the footer.
    let mut truncated = image.clone();
    let cut = truncated.len() / 2;
    truncated.remove(cut);
    let refused = match parse_image_footer(&truncated) {
        Some((k, t)) => !verify_kernel_attestation(&root, &k, &t),
        None => true, // a footer that no longer parses is a refusal
    };
    out.push(Finding {
        id: "KRN-TRUNC",
        category: "integrity",
        severity: Severity::Critical,
        description: "drop a byte from the image, corrupting the layout",
        refused,
    });

    // impersonation: a foreign kernel shipped under a stolen trailer.
    let foreign = b"a malicious kernel that was never enrolled".to_vec();
    let img = assemble_image(&foreign, trailer.clone());
    let (k, t) = parse_image_footer(&img).expect("foreign image parses");
    out.push(Finding {
        id: "KRN-SWAP",
        category: "impersonation",
        severity: Severity::Critical,
        description: "swap a foreign kernel under a stolen trailer",
        refused: !verify_kernel_attestation(&root, &k, &t),
    });

    // forgery: the attacker enrolls their own kernel under their own root.
    let (_attacker_root, forged) = enroll_kernel(&foreign);
    let img = assemble_image(&foreign, forged);
    let (k, t) = parse_image_footer(&img).expect("forged image parses");
    out.push(Finding {
        id: "KRN-FORGE",
        category: "forgery",
        severity: Severity::Critical,
        description: "forge a trailer under a different root",
        refused: !verify_kernel_attestation(&root, &k, &t),
    });

    // forgery: a single bit flipped inside the serialized trailer.
    let mut bad_trailer = trailer.clone();
    let mid = bad_trailer.len() / 2;
    bad_trailer[mid] ^= 0x80;
    let img = assemble_image(&genuine, bad_trailer);
    let refused = match parse_image_footer(&img) {
        Some((k, t)) => !verify_kernel_attestation(&root, &k, &t),
        None => true,
    };
    out.push(Finding {
        id: "TRL-FLIP",
        category: "forgery",
        severity: Severity::Critical,
        description: "flip a bit inside the STARK trailer",
        refused,
    });

    // malformed: an image too short to hold a footer must be refused, not indexed.
    let runt = vec![0u8; 8];
    out.push(Finding {
        id: "IMG-RUNT",
        category: "malformed",
        severity: Severity::High,
        description: "an undersized image must be refused, not panic",
        refused: parse_image_footer(&runt).is_none(),
    });

    // impersonation: the trailer is bound to the kernel context, so an empty
    // kernel cannot ride a real trailer.
    let empty = b"".to_vec();
    out.push(Finding {
        id: "KRN-EMPTY",
        category: "impersonation",
        severity: Severity::Critical,
        description: "an empty kernel cannot ride a real trailer",
        refused: !verify_kernel_attestation(&root, &empty, &trailer),
    });

    out
}
