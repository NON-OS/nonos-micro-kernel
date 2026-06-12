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

use alloc::vec;
use alloc::vec::Vec;

use crate::term::util::format_u64;

pub(super) fn wc(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut buf = [0u8; 24];
    let n = format_u64(input.len() as u64, &mut buf);
    vec![buf[..n].to_vec()]
}

pub(super) fn head(arg: Option<&&[u8]>, input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    input.into_iter().take(parse_n(arg)).collect()
}

pub(super) fn tail(arg: Option<&&[u8]>, input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let skip = input.len().saturating_sub(parse_n(arg));
    input.into_iter().skip(skip).collect()
}

fn parse_n(arg: Option<&&[u8]>) -> usize {
    arg.and_then(|a| core::str::from_utf8(a).ok())
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10)
}
