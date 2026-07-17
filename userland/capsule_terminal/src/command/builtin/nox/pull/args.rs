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

use super::target::{default_dest, parse_target, Target};

pub struct PullArgs {
    pub target: Target,
    pub dest: Vec<u8>,
    pub no_clobber: bool,
    pub skip_unchanged: bool,
    pub auth: Option<Vec<u8>>,
    pub headers: Vec<Vec<u8>>,
}

pub fn parse(argv: &[&[u8]]) -> Result<PullArgs, &'static str> {
    let mut no_clobber = false;
    let mut skip_unchanged = false;
    let mut auth = None;
    let mut headers = Vec::new();
    let mut rest = argv;
    while let Some((&first, tail)) = rest.split_first() {
        match first {
            b"-n" => {
                no_clobber = true;
                rest = tail;
            }
            b"-u" => {
                skip_unchanged = true;
                rest = tail;
            }
            b"-a" => {
                let (&v, t) = tail.split_first().ok_or("-a needs user:pass")?;
                auth = Some(v.to_vec());
                rest = t;
            }
            b"-H" => {
                let (&v, t) = tail.split_first().ok_or("-H needs 'Key: Value'")?;
                headers.push(v.to_vec());
                rest = t;
            }
            _ => break,
        }
    }
    if rest.is_empty() || rest.len() > 2 {
        return Err("usage: nox pull [-n] [-u] [-a user:pass] [-H 'K: V'] <host[:port]>/<path> [dest]");
    }
    let target = parse_target(rest[0])?;
    let dest = match rest.get(1) {
        Some(d) => d.to_vec(),
        None => default_dest(&target),
    };
    Ok(PullArgs { target, dest, no_clobber, skip_unchanged, auth, headers })
}
