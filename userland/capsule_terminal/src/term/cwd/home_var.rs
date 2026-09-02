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

use crate::term::state::State;

/// The tab's `$HOME`. `State::new` seeds the variable with `cwd::HOME`, so this
/// normally answers that; a script that unsets it gets an empty slice back and
/// the prompt falls through to the absolute path rather than inventing a home.
pub fn home_var(state: &State) -> &[u8] {
    for (k, v) in state.vars.iter() {
        if k.as_slice() == b"HOME" {
            return v.as_slice();
        }
    }
    b""
}
