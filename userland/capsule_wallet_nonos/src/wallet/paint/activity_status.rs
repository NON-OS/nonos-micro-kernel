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

use crate::wallet::state::State;
use crate::wallet::theme::{FG, GREEN, MUTED, WARN};

/// Where a transaction has got to, in the words the reader needs.
///
/// Four states because there are four: a signature exists, it reached the
/// network, and the chain either took it or rejected it. Collapsing the last
/// two would let a reverted transfer read as a sent one.
pub fn status_of(state: &State) -> (&'static str, u32) {
    if state.receipt_ready {
        if state.receipt_ok {
            ("confirmed", GREEN())
        } else {
            ("reverted", WARN())
        }
    } else if state.broadcast_ready {
        ("sent, waiting", FG())
    } else {
        ("signed", MUTED())
    }
}
