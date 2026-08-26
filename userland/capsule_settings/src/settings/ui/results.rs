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

//! Search walks every section rather than the open one, so a query answers
//! "where does NONOS keep this?" instead of filtering what is already on screen.
//! The tables are small enough that recomputing per frame beats caching a list.

use nonos_policy_proto::{label_of, Field};

use crate::settings::schema::{field_at, field_count};
use crate::settings::section::{Section, SECTIONS};

pub fn count(query: &str) -> usize {
    let mut n = 0;
    for section in SECTIONS {
        for i in 0..field_count(section) {
            if field_at(section, i).is_some_and(|f| matches(f, query)) {
                n += 1;
            }
        }
    }
    n
}

pub fn at(query: &str, want: usize) -> Option<(Section, usize, Field)> {
    let mut n = 0;
    for section in SECTIONS {
        for i in 0..field_count(section) {
            let Some(field) = field_at(section, i) else { continue };
            if !matches(field, query) {
                continue;
            }
            if n == want {
                return Some((section, i, field));
            }
            n += 1;
        }
    }
    None
}

fn matches(field: Field, query: &str) -> bool {
    let label = label_of(field);
    let q = query.as_bytes();
    if q.is_empty() || q.len() > label.len() {
        return false;
    }
    label.windows(q.len()).any(|w| eq_fold(w, q))
}

fn eq_fold(a: &[u8], b: &[u8]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.eq_ignore_ascii_case(y))
}
