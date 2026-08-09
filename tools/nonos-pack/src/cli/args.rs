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

use std::path::Path;

use nonos_capsule_sign::algs::{parse_alg, AlgId};
use nonos_capsule_sign::keys::read_seed;

pub fn flag(cmd: &str, av: &[String], name: &str) -> Result<String, String> {
    let i = av
        .iter()
        .position(|a| a == name)
        .ok_or_else(|| format!("{}: missing {} <path>", cmd, name))?;
    av.get(i + 1).cloned().ok_or_else(|| format!("{}: {} needs a value", cmd, name))
}

pub fn seeds(cmd: &str, av: &[String]) -> Result<(Vec<u8>, Vec<u8>), String> {
    let (mut ed, mut mldsa) = (None, None);
    for (i, a) in av.iter().enumerate() {
        if a != "--seed" {
            continue;
        }
        let spec = av.get(i + 1).ok_or_else(|| format!("{}: --seed alg=<path>", cmd))?;
        let (label, path) = spec
            .split_once('=')
            .ok_or_else(|| format!("{}: expected alg=path, got `{}`", cmd, spec))?;
        let alg = parse_alg(label).map_err(|e| format!("{}: {}", cmd, e))?;
        let km = read_seed(Path::new(path)).map_err(|e| format!("{}: {}", path, e))?;
        if km.alg != alg {
            return Err(format!("{}: {} is not a {} seed", cmd, path, label));
        }
        match alg {
            AlgId::Ed25519 => ed = Some(km.bytes),
            AlgId::MlDsa65 => mldsa = Some(km.bytes),
            other => return Err(format!("{}: unsupported seed alg {}", cmd, other.label())),
        }
    }
    Ok((
        ed.ok_or_else(|| format!("{}: missing --seed ed25519=<path>", cmd))?,
        mldsa.ok_or_else(|| format!("{}: missing --seed mldsa65=<path>", cmd))?,
    ))
}
