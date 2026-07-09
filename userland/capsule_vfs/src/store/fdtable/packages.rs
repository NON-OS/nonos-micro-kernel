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

// The real, unmodified ripgrep v14.1.1 from crates.io (stripped to fit the
// load cap), staged so `install rg <pattern> <path>` runs it against the VFS.
const RIPGREP: [(&str, &[u8]); 4] = [
    (
        "/capsules/rg.elf",
        include_bytes!("../../../../capsule_ripgrep/target/x86_64-nonos-user/release/rg"),
    ),
    (
        "/capsules/rg.nonos_id_cert.bin",
        include_bytes!("../../../../../nonos-data/trust/capsules/rg.nonos_id_cert.bin"),
    ),
    (
        "/capsules/rg.manifest.bin",
        include_bytes!("../../../../../nonos-data/trust/capsules/rg.manifest.bin"),
    ),
    (
        "/capsules/rg.zk_trailer.bin",
        include_bytes!("../../../../../nonos-data/trust/capsules/rg.zk_trailer.bin"),
    ),
];

impl Store {
    pub(super) fn seed_packages(&mut self) {
        for &(name, data) in STD_PROOF.iter().chain(RIPGREP.iter()) {
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
