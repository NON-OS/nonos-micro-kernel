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

use crate::dns::{HDR_LEN, LABEL_MAX, NAME_MAX, POINTER_MASK};

pub fn matches(query: &[u8], response: &[u8]) -> bool {
    let q_end = match name_end(query, HDR_LEN) {
        Some(p) => p,
        None => return false,
    };
    let r_end = match name_end(response, HDR_LEN) {
        Some(p) => p,
        None => return false,
    };
    if !names_equal(&query[HDR_LEN..q_end], &response[HDR_LEN..r_end]) {
        return false;
    }
    q_end + 2 <= query.len()
        && r_end + 2 <= response.len()
        && query[q_end..q_end + 2] == response[r_end..r_end + 2]
}

fn name_end(message: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    let mut steps = 0;
    loop {
        let b = *message.get(i)?;
        if b == 0 {
            return Some(i + 1);
        }
        if b & POINTER_MASK == POINTER_MASK || b as usize > LABEL_MAX {
            return None;
        }
        i += 1 + b as usize;
        steps += 1;
        if i > message.len() || steps > NAME_MAX {
            return None;
        }
    }
}

fn names_equal(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b.iter()).all(|(x, y)| fold(*x) == fold(*y))
}

fn fold(b: u8) -> u8 {
    if b.is_ascii_uppercase() {
        b + 32
    } else {
        b
    }
}
