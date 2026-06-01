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

use std::fs;

use nonos_registry::{add, find, load, remove, RegistryEntry};

fn entry(name: &str) -> RegistryEntry {
    RegistryEntry {
        name: name.to_string(),
        version: (1, 0, 0),
        target: "x86_64-nonos-user".to_string(),
        namespace: format!("systems.nonos.{name}"),
        cert_fingerprint: "abcd1234".to_string(),
    }
}

#[test]
fn add_find_remove_roundtrip_is_reproducible() {
    let dir = std::env::temp_dir().join("nonos_reg_test");
    let _ = fs::create_dir_all(&dir);
    let index = dir.join("registry.index");
    let _ = fs::remove_file(&index);

    add(&index, entry("beta")).unwrap();
    add(&index, entry("alpha")).unwrap();
    let first = fs::read_to_string(&index).unwrap();

    let _ = fs::remove_file(&index);
    add(&index, entry("alpha")).unwrap();
    add(&index, entry("beta")).unwrap();
    let second = fs::read_to_string(&index).unwrap();
    assert_eq!(first, second, "index must be order-independent and reproducible");

    assert_eq!(load(&index).len(), 2);
    assert!(find(&index, "alpha").is_some());
    assert!(remove(&index, "alpha").unwrap());
    assert!(find(&index, "alpha").is_none());
    assert_eq!(load(&index).len(), 1);
}
