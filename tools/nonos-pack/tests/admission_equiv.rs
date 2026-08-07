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

mod fixtures;

static SCRATCH_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn lock_scratch() -> std::sync::MutexGuard<'static, ()> {
    SCRATCH_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn packed_then_unpacked_artifacts_still_admit() {
    let _scratch = lock_scratch();
    let f = match fixtures::gui_demo_paths() {
        Some(f) => f,
        None => {
            eprintln!("skip: no gui_demo artifacts");
            return;
        }
    };
    let pkg = fixtures::pack_gui_demo(&f);
    let out = fixtures::unpack(&pkg);
    assert_eq!(std::fs::read(&out.manifest).unwrap(), std::fs::read(&f.manifest).unwrap());
    assert_eq!(std::fs::read(&out.id_cert).unwrap(), std::fs::read(&f.id_cert).unwrap());
    assert_eq!(std::fs::read(&out.elf).unwrap(), std::fs::read(&f.elf).unwrap());
    assert_eq!(std::fs::read(&out.trailer).unwrap(), std::fs::read(&f.trailer).unwrap());
    let status = std::process::Command::new(fixtures::capsule_sign())
        .args(["verify-manifest", "--manifest"])
        .arg(&out.manifest)
        .arg("--cert")
        .arg(&out.id_cert)
        .arg("--policy")
        .arg(fixtures::trust_policy())
        .status()
        .unwrap();
    assert!(status.success(), "unpacked artifacts must pass verify-manifest");
}

#[test]
fn core_parse_sections_match_container_decode_byte_for_byte() {
    let _scratch = lock_scratch();
    let f = match fixtures::gui_demo_paths() {
        Some(f) => f,
        None => {
            eprintln!("skip: no gui_demo artifacts");
            return;
        }
    };
    let pkg = fixtures::pack_gui_demo(&f);
    let bytes = std::fs::read(&pkg.path).unwrap();
    let (c, off) = nonos_pack::container::decode(&bytes).unwrap();
    let (s, core_off) = nonos_pack_core::parse(&bytes).unwrap();
    assert_eq!(off, core_off, "signed-region end must agree");
    use nonos_pack::container::{section, SectionKind};
    assert_eq!(section(&c, SectionKind::Manifest).unwrap(), s.manifest);
    assert_eq!(section(&c, SectionKind::Elf).unwrap(), s.elf);
    assert_eq!(section(&c, SectionKind::IdCert).unwrap(), s.id_cert);
    assert_eq!(section(&c, SectionKind::ZkTrailer).unwrap(), s.zk_trailer);
    nonos_pack_core::check_trailer(&bytes, core_off).expect("sealed pack has canonical trailer");
}
