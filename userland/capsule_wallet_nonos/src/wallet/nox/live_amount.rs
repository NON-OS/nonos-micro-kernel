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

use super::amount_str;

// Like amount_str, but distinguishes "still loading" from "unavailable": when the
// value has not arrived yet, show an ellipsis if the chain link is up (a read is
// coming) and an em dash only when there is no route. This keeps a fresh wallet
// looking alive and refreshing instead of dead.
pub fn live_amount<'a>(ready: bool, wei: &[u8; 32], link_up: bool, buf: &'a mut [u8]) -> &'a str {
    if !ready {
        return if link_up { "\u{2026}" } else { "\u{2014}" };
    }
    amount_str(ready, wei, buf)
}
