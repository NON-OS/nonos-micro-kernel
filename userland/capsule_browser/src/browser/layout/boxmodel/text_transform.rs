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

use alloc::string::{String, ToString};

use crate::browser::css::TextTransform;

// Apply a text-transform to one word before it is measured and drawn, so the
// glyphs the layout sizes match the ones painted. Capitalize upper-cases the
// first character of the word and leaves the rest untouched.
pub(super) fn transform(w: &str, mode: TextTransform) -> String {
    match mode {
        TextTransform::None => w.to_string(),
        TextTransform::Upper => w.to_uppercase(),
        TextTransform::Lower => w.to_lowercase(),
        TextTransform::Capitalize => {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        }
    }
}
