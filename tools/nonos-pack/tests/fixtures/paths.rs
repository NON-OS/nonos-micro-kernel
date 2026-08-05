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

use std::path::PathBuf;

pub struct GuiDemoPaths {
    pub manifest: PathBuf,
    pub elf: PathBuf,
    pub id_cert: PathBuf,
    pub trailer: PathBuf,
    pub ed_seed: PathBuf,
    pub mldsa_seed: PathBuf,
}

pub fn gui_demo_paths() -> Option<GuiDemoPaths> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let trust = root.join("nonos-data").join("trust").join("capsules");
    let keys = root.join(".keys");
    let p = GuiDemoPaths {
        manifest: trust.join("gui_demo.manifest.bin"),
        elf: root.join("userland/capsule_gui_demo/target/x86_64-nonos-user/release/gui_demo"),
        id_cert: trust.join("gui_demo.nonos_id_cert.bin"),
        trailer: trust.join("gui_demo.zk_trailer.bin"),
        ed_seed: keys.join("gui_demo_publisher_ed25519.seed"),
        mldsa_seed: keys.join("gui_demo_publisher_mldsa65.seed"),
    };
    let all = [&p.manifest, &p.elf, &p.id_cert, &p.trailer, &p.ed_seed, &p.mldsa_seed];
    if all.iter().all(|f| f.is_file()) {
        Some(p)
    } else {
        None
    }
}
