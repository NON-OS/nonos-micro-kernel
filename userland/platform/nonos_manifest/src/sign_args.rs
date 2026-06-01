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

use super::caps::cap_mask;
use super::endpoint::EndpointKind;
use super::model::Manifest;

pub fn build_sign_args(m: &Manifest, elf: &str, out: &str) -> Result<Vec<String>, String> {
    let required = cap_mask(&m.required_caps)?;
    let optional = cap_mask(&m.optional_caps)?;
    let mut a = vec![
        "sign-manifest".to_string(),
        "--cert".into(), m.cert.clone(),
        "--namespace".into(), m.namespace.clone(),
        "--version".into(), format!("{}.{}.{}", m.version.0, m.version.1, m.version.2),
        "--target".into(), m.target.clone(),
        "--elf".into(), elf.to_string(),
        "--required-caps".into(), format!("0x{required:x}"),
        "--optional-caps".into(), format!("0x{optional:x}"),
    ];
    for e in &m.endpoints {
        let kind = match e.kind {
            EndpointKind::Service => "service",
            EndpointKind::Reply => "reply",
        };
        a.push("--endpoint".into());
        a.push(format!("{}:{}:{}", kind, e.port, e.name));
    }
    for (alg, path) in &m.pub_seeds {
        a.push("--pub-seed".into());
        a.push(format!("{alg}={path}"));
    }
    a.push("--out".into());
    a.push(out.to_string());
    Ok(a)
}
