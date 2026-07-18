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

//! nonos-defend: verify an attested image before you flash it. It parses the
//! image footer and checks the kernel's transparent STARK self-attestation
//! against the enrolled root, the same check the bootloader runs before jump.
//! Exit 0 means the image is genuine; non-zero means do not flash it.

use nonos_secops::{parse_image_footer, verify_kernel_attestation};
use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: nonos-defend <attested-image> <kernel-attest-root.bin>");
        exit(2);
    }
    let image = fs::read(&args[1]).unwrap_or_else(|e| {
        eprintln!("cannot read image {}: {e}", args[1]);
        exit(2)
    });
    let root_bytes = fs::read(&args[2]).unwrap_or_else(|e| {
        eprintln!("cannot read root {}: {e}", args[2]);
        exit(2)
    });
    if root_bytes.len() != 32 {
        eprintln!("root must be 32 bytes, got {}", root_bytes.len());
        exit(2);
    }
    let root: [u8; 32] = root_bytes.try_into().unwrap();

    let Some((kernel_bytes, trailer)) = parse_image_footer(&image) else {
        eprintln!("REJECTED: no attestation footer in the image");
        exit(1);
    };

    if verify_kernel_attestation(&root, &kernel_bytes, &trailer) {
        println!("VERIFIED: the kernel self-attestation holds against the enrolled root");
        println!("  kernel region: {} bytes", kernel_bytes.len());
        println!("  trailer:       {} bytes", trailer.len());
        exit(0);
    } else {
        eprintln!("REJECTED: the kernel self-attestation does not verify against this root");
        exit(1);
    }
}
