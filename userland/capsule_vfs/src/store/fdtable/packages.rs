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

use alloc::string::String;
use alloc::vec::Vec;

use super::types::{File, Store, MAX_FILES};

// Packages staged into the store at boot so they can be verified, loaded, and
// spawned at runtime through `install <name>`, instead of being baked into the
// kernel image and spawned at boot. Each is the four signed artifacts the
// installer reads as /capsules/<name>.{elf,nonos_id_cert.bin,manifest.bin,
// zk_trailer.bin}. This stands in for a future network fetch; the install path
// itself (verify, load, spawn) is identical either way.
// std_proof is built on the std platform layer, which issues its syscalls with
// raw x86_64 registers and so has no aarch64 build to stage. There the store
// starts empty instead; the runtime install path is the same either way.
#[cfg(target_arch = "x86_64")]
const STD_PROOF: [(&str, &[u8]); 4] = [
    (
        "/capsules/std_proof.elf",
        include_bytes!(
            "../../../../capsule_std_proof/target/x86_64-nonos-user/release/std_proof"
        ),
    ),
    (
        "/capsules/std_proof.nonos_id_cert.bin",
        include_bytes!("../../../../../nonos-data/trust/capsules/std_proof.nonos_id_cert.bin"),
    ),
    (
        "/capsules/std_proof.manifest.bin",
        include_bytes!("../../../../../nonos-data/trust/capsules/std_proof.manifest.bin"),
    ),
    (
        "/capsules/std_proof.zk_trailer.bin",
        include_bytes!("../../../../../nonos-data/trust/capsules/std_proof.zk_trailer.bin"),
    ),
];

// NOTE: ripgrep (rg.elf ~24 MB) is intentionally NOT baked into the store. The
// vfs capsule heap is 16 MB, so staging a 24 MB file at boot exhausts it and the
// capsule aborts before it can serve a single request, which made the whole
// filesystem read as "vfs ipc failed". Large packages belong to the runtime
// installer path (`/capsules` starts empty and is filled on demand), not the
// image. std_proof (~1.5 MB) is small enough to stay as a working demo.

#[cfg(not(target_arch = "x86_64"))]
const STD_PROOF: [(&str, &[u8]); 0] = [];

impl Store {
    pub(super) fn seed_packages(&mut self) {
        for &(name, data) in STD_PROOF.iter() {
            self.stage(name, data);
        }
    }

    fn stage(&mut self, name: &str, data: &[u8]) {
        if self.files.len() >= MAX_FILES || self.files.iter().any(|f| f.name == name) {
            return;
        }
        self.files.push(File::new(String::from(name), Vec::from(data), false));
    }
}
