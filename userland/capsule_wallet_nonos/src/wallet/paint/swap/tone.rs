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

//! Which colour a price impact is worth.

use crate::wallet::swap::{is_dangerous, is_warning};
use crate::wallet::theme::{CYAN, GREEN, WARN};

/// Green while a trade is unremarkable, cyan once it is worth noticing,
/// and the warning colour once it stops being a trade at all.
pub fn impact_tone(bps: u32) -> u32 {
    if is_dangerous(bps) {
        WARN()
    } else if is_warning(bps) {
        CYAN()
    } else {
        GREEN()
    }
}
