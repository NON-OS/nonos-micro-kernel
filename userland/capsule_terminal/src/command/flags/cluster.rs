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

//! Walk one clustered short-flag group such as `-rf` or `-n5`.

use alloc::vec::Vec;

use super::err::{missing, unknown};
use super::spec::{Parsed, Spec};

pub(super) fn cluster<'a>(
    spec: &Spec,
    arg: &'a [u8],
    body: &'a [u8],
    args: &[&'a [u8]],
    mut i: usize,
    out: &mut Parsed<'a>,
) -> Result<usize, Vec<u8>> {
    let mut j = 0;
    while j < body.len() {
        let c = body[j];
        j += 1;
        if spec.valued.contains(&c) {
            let rest = &body[j..];
            if rest.is_empty() {
                let Some(&val) = args.get(i) else { return Err(missing(spec, arg)) };
                i += 1;
                out.vals.push((c, val));
            } else {
                out.vals.push((c, rest));
            }
            return Ok(i);
        }
        if !spec.bools.contains(&c) {
            return Err(unknown(spec, c));
        }
        out.seen.push(c);
    }
    Ok(i)
}
