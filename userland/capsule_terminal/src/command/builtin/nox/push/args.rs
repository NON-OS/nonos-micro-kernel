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
}

pub fn parse(argv: &[&[u8]]) -> Result<PushArgs, &'static str> {
    if argv.len() != 2 {
        return Err("usage: nox push <local> <host[:port]>/<path>");
    }
    let src = argv[0].to_vec();
    let target = parse_target(argv[1])?;
    Ok(PushArgs { src, target })
}
