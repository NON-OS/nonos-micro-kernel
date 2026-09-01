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

//! Split an argument list into flags and operands.

use alloc::vec::Vec;

use super::cluster::cluster;
use super::err::missing;
use super::spec::{Parsed, Spec};

pub fn parse<'a>(spec: &Spec, args: &[&'a [u8]]) -> Result<Parsed<'a>, Vec<u8>> {
    let mut out = Parsed::default();
    let mut literal = false;
    let mut i = 0;
    while i < args.len() {
        let arg = args[i];
        i += 1;
        if literal || arg.len() < 2 || arg[0] != b'-' {
            out.operands.push(arg);
            continue;
        }
        if arg == b"--" {
            literal = true;
            continue;
        }
        let body = &arg[1..];
        if spec.words.iter().any(|w| *w == body) {
            let Some(&val) = args.get(i) else { return Err(missing(spec, arg)) };
            i += 1;
            out.wvals.push((body, val));
            continue;
        }
        if spec.numeric != 0 && body.iter().all(u8::is_ascii_digit) {
            out.vals.push((spec.numeric, body));
            continue;
        }
        i = cluster(spec, arg, body, args, i, &mut out)?;
    }
    Ok(out)
}
