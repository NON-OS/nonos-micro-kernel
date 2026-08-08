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

use super::q32_to_u128::q32_to_u128;

/// Wei in one whole NOX.
pub const WEI_PER_NOX: u128 = 1_000_000_000_000_000_000;

/// Decimal places the token divides into.
pub const NOX_DECIMALS: u32 = 18;

/// What this wallet holds, in wei, or `None` while the balance has not been
/// read back from the chain.
///
/// The distinction matters: a balance that has not arrived is not a balance
/// of zero. Staking decides what to sign from this, and signing against a
/// figure nobody has read would put a transaction on chain that can only
/// fail, at the reader's expense.
///
/// Reported in wei rather than whole tokens so staking everything means
/// everything. Truncating to whole NOX would strand the fraction, and on a
/// balance that grows by rewards the fraction is most of what is new.
pub fn held_wei(ready: bool, wei: &[u8; 32]) -> Option<u128> {
    if !ready {
        return None;
    }
    q32_to_u128(wei)
}
