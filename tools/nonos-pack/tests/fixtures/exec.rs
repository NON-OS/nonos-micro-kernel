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
use std::process::Command;

use super::paths::GuiDemoPaths;

pub struct Packed {
    pub path: PathBuf,
}
pub struct Unpacked { pub manifest: PathBuf, pub id_cert: PathBuf, pub elf: PathBuf, pub trailer: PathBuf }

fn repo_root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..") }
pub fn nonos_pack_bin() -> PathBuf { repo_root().join("tools/nonos-pack/target/release/nonos-pack") }
pub fn capsule_sign() -> PathBuf { repo_root().join("nonos-sign/target/release/capsule-sign") }
pub fn trust_policy() -> PathBuf { repo_root().join("nonos-data/trust/policy/nonos_trust_anchor.policy.bin") }

fn scratch_dir(tag: &str) -> PathBuf {
    let base = std::env::temp_dir();
    let dir = base.join(format!("nonos-pack-admission-equiv-{}-{}", tag, std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn pack_gui_demo(f: &GuiDemoPaths) -> Packed {
    let out = scratch_dir("pack").join("gui_demo.nonos");
    let status = Command::new(nonos_pack_bin())
        .arg("pack")
        .arg("--out").arg(&out)
        .arg("--manifest").arg(&f.manifest)
        .arg("--elf").arg(&f.elf)
        .arg("--id-cert").arg(&f.id_cert)
        .arg("--trailer").arg(&f.trailer)
        .arg("--seed").arg(format!("ed25519={}", f.ed_seed.display()))
        .arg("--seed").arg(format!("mldsa65={}", f.mldsa_seed.display()))
        .status()
        .expect("failed to run nonos-pack pack");
    assert!(status.success(), "nonos-pack pack failed");
    Packed { path: out }
}

pub fn unpack(pkg: &Packed) -> Unpacked {
    let dir = scratch_dir("unpack");
    let status = Command::new(nonos_pack_bin())
        .arg("unpack")
        .arg("--in").arg(&pkg.path)
        .arg("--out-dir").arg(&dir)
        .status()
        .expect("failed to run nonos-pack unpack");
    assert!(status.success(), "nonos-pack unpack failed");
    Unpacked {
        manifest: dir.join("gui_demo.manifest.bin"),
        id_cert: dir.join("gui_demo.nonos_id_cert.bin"),
        elf: dir.join("gui_demo.elf"),
        trailer: dir.join("gui_demo.zk_trailer.bin"),
    }
}
