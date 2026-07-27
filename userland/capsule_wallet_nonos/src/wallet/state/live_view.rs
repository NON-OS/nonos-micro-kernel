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

use super::types::{VIEW_HOME, VIEW_NOX, VIEW_SEND, VIEW_SHIELDED};

/// Whether a screen shows live on-chain data and so warrants the background
/// probe. Receive (address, QR, account setup) and Proof (a static record of
/// the last signed transaction) do not, so the probe never runs there and its
/// blocking network read cannot stall those flows.
pub fn needs_live_data(view: u8) -> bool {
    matches!(view, VIEW_HOME | VIEW_SEND | VIEW_NOX | VIEW_SHIELDED)
}
