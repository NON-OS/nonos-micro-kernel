// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use std::{collections::BTreeMap, path::PathBuf};

pub fn require_same_names(
    capsules: &BTreeMap<String, PathBuf>,
    sidecars: &BTreeMap<String, PathBuf>,
) -> Result<(), String> {
    for name in capsules.keys() {
        if !sidecars.contains_key(name) {
            return Err(format!("missing ZK sidecar for capsule: {name}"));
        }
    }
    for name in sidecars.keys() {
        if !capsules.contains_key(name) {
            return Err(format!("orphan ZK sidecar without capsule blob: {name}"));
        }
    }
    Ok(())
}
