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

//! Character references, resolved to the characters they name.
//!
//! These used to fold onto ASCII lookalikes: an em dash arrived as two
//! hyphens, a copyright sign as three characters in brackets, a non
//! breaking space as an ordinary one. The bundled face carries all of them,
//! so the substitution only ever cost fidelity and changed how lines wrap.
//!
//! Anything unrecognised is written back out as it arrived. A page carrying
//! a reference this does not know is better read with the source visible
//! than with the text around it silently dropped.

mod core;
mod greek;
mod latin;
mod math;
mod numeric;
mod punctuation;
mod symbols;

use alloc::string::String;

pub fn push_decoded(out: &mut String, entity: &str) {
    if let Some(c) = numeric::numeric(entity) {
        out.push(c);
        return;
    }
    let found = core::core(entity)
        .or_else(|| punctuation::punctuation(entity))
        .or_else(|| symbols::symbols(entity))
        .or_else(|| math::math(entity))
        .or_else(|| latin::latin_lower(entity))
        .or_else(|| latin::latin_upper(entity))
        .or_else(|| greek::greek(entity));
    match found {
        Some(s) => out.push_str(s),
        None => {
            out.push('&');
            out.push_str(entity);
            out.push(';');
        }
    }
}
