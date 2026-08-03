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
//! The delta path, against a pack GitHub served for a real repository.
//!
//! The pack is too large to vendor, so this reads one from an environment
//! variable and skips when it is absent. Point `NONOS_GIT_TEST_PACK` at a
//! `.pack` from any clone and every id it holds is checked against the id we
//! recompute from the bytes we reconstructed, deltas resolved and all.

use std::collections::BTreeSet;

use nonos_git::read_pack;

#[test]
fn every_object_in_a_real_delta_pack_reconstructs() {
    let Ok(path) = std::env::var("NONOS_GIT_TEST_PACK") else {
        eprintln!("skipping: set NONOS_GIT_TEST_PACK to a .pack file");
        return;
    };
    let data = std::fs::read(&path).expect("read pack");
    let objects = read_pack(&data).expect("real pack must read");

    // Every id we hand back is the SHA-1 of the content we rebuilt, so a set
    // matching git's own listing means every delta chain resolved correctly.
    let ours: BTreeSet<String> = objects.iter().map(|o| o.id.to_hex()).collect();

    let expected = std::env::var("NONOS_GIT_TEST_IDS").ok();
    let Some(ids_path) = expected else {
        eprintln!("read {} objects from {}", objects.len(), path);
        return;
    };
    let listed = std::fs::read_to_string(ids_path).expect("read ids");
    let theirs: BTreeSet<String> =
        listed.lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect();

    let missing: Vec<&String> = theirs.difference(&ours).take(5).collect();
    assert!(
        missing.is_empty(),
        "{} objects git has that we did not rebuild: {missing:?}",
        theirs.difference(&ours).count()
    );
    assert_eq!(ours.len(), theirs.len(), "object count differs");
}
