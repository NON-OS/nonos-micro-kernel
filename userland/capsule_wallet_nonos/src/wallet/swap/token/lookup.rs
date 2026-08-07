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

//! Reaching a token by position.

use super::kind::Token;
use super::list::TOKENS;

/// The token at `index`, wrapping so a cycling control cannot run off the
/// end of the list.
pub fn token(index: u8) -> &'static Token {
    &TOKENS[(index as usize) % TOKENS.len()]
}

/// How many tokens are on the list.
pub fn count() -> u8 {
    TOKENS.len() as u8
}
