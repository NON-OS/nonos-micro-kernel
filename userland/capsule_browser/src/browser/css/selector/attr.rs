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

use alloc::string::String;

// An attribute test: presence or one of the CSS match operators.
pub enum AttrTest {
    Present,
    // = exact value
    Eq(String),
    // *= substring
    Contains(String),
    // ^= prefix
    Starts(String),
    // $= suffix
    Ends(String),
    // ~= whitespace-separated word
    Word(String),
    // |= exact or followed by a hyphen
    Lang(String),
}

impl AttrTest {
    pub fn matches(&self, have: &str) -> bool {
        match self {
            AttrTest::Present => true,
            AttrTest::Eq(v) => have == v,
            AttrTest::Contains(v) => !v.is_empty() && have.contains(v.as_str()),
            AttrTest::Starts(v) => !v.is_empty() && have.starts_with(v.as_str()),
            AttrTest::Ends(v) => !v.is_empty() && have.ends_with(v.as_str()),
            AttrTest::Word(v) => have.split_whitespace().any(|w| w == v),
            AttrTest::Lang(v) => {
                have == v || have.strip_prefix(v.as_str()).is_some_and(|r| r.starts_with('-'))
            }
        }
    }
}
