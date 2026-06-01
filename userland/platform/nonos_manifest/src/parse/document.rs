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

use super::array::parse_str_array;
use super::endpoint::finish_endpoint;
use super::strings::unquote;
use super::version::parse_version;
use crate::model::Manifest;

pub fn parse_document(text: &str) -> Result<Manifest, String> {
    let mut m = Manifest {
        name: String::new(), namespace: String::new(), cert: String::new(),
        version: (0, 0, 0), target: String::new(), required_caps: Vec::new(),
        optional_caps: Vec::new(), endpoints: Vec::new(), pub_seeds: Vec::new(),
    };
    let mut ep: Option<(String, u32, String)> = None;
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if line == "[[endpoint]]" {
            if let Some(e) = ep.take() {
                m.endpoints.push(finish_endpoint(e)?);
            }
            ep = Some((String::new(), 0, String::new()));
            continue;
        }
        let (k, v) = line.split_once('=').ok_or_else(|| format!("bad line: {line}"))?;
        let (k, v) = (k.trim(), v.trim());
        if let Some(e) = ep.as_mut() {
            match k {
                "kind" => e.0 = unquote(v)?,
                "port" => e.1 = v.parse().map_err(|_| format!("bad port: {v}"))?,
                "name" => e.2 = unquote(v)?,
                other => return Err(format!("unknown endpoint key: {other}")),
            }
            continue;
        }
        match k {
            "name" => m.name = unquote(v)?,
            "namespace" => m.namespace = unquote(v)?,
            "cert" => m.cert = unquote(v)?,
            "target" => m.target = unquote(v)?,
            "version" => m.version = parse_version(&unquote(v)?)?,
            "required_caps" => m.required_caps = parse_str_array(v)?,
            "optional_caps" => m.optional_caps = parse_str_array(v)?,
            "pub_seed_ed25519" => m.pub_seeds.push(("ed25519".to_string(), unquote(v)?)),
            "pub_seed_mldsa65" => m.pub_seeds.push(("mldsa65".to_string(), unquote(v)?)),
            other => return Err(format!("unknown key: {other}")),
        }
    }
    if let Some(e) = ep.take() {
        m.endpoints.push(finish_endpoint(e)?);
    }
    Ok(m)
}
