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

use alloc::vec::Vec;

use super::super::pull::target::{parse_target, Target};

pub struct PushArgs {
    pub src: Vec<u8>,
    pub target: Target,
    pub auth: Option<Vec<u8>>,
    pub headers: Vec<Vec<u8>>,
}

pub fn parse(argv: &[&[u8]]) -> Result<PushArgs, &'static str> {
    let mut auth = None;
    let mut headers = Vec::new();
    let mut rest = argv;
    while let Some((&first, tail)) = rest.split_first() {
        match first {
            b"-a" => {
                let (&v, t) = tail.split_first().ok_or("-a needs user:pass")?;
                auth = Some(v.to_vec());
                rest = t;
            }
            b"-H" => {
                let (&v, t) = tail.split_first().ok_or("-H needs 'Key: Value'")?;
                if v.iter().any(|&b| b == b'\r' || b == b'\n') {
                    return Err("-H value must not contain CR or LF");
                }
                headers.push(v.to_vec());
                rest = t;
            }
            _ => break,
        }
    }
    if rest.len() != 2 {
        return Err("usage: nox push [-a user:pass] [-H 'K: V'] <local> <host[:port]>/<path>");
    }
    let src = rest[0].to_vec();
    let target = parse_target(rest[1])?;
    Ok(PushArgs { src, target, auth, headers })
}
