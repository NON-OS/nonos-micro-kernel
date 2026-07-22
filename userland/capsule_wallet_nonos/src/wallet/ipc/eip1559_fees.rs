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

const GWEI: u64 = 1_000_000_000;

/// Derive EIP-1559 (max_priority_fee, max_fee) from the live gas price the
/// wallet fetched over eth_gasPrice. The cap carries headroom so the transfer
/// stays includable even if the base fee rises before it is mined; a zero
/// price (fee not yet fetched) falls back to a safe fixed level.
pub fn eip1559_fees(gas_price_wei: u64) -> (u128, u128) {
    let base = if gas_price_wei == 0 { 15 * GWEI } else { gas_price_wei };
    // Tip: a tenth of the observed price, clamped to a practical band.
    let priority = (base / 10).clamp(GWEI, 3 * GWEI);
    // Cap at twice the observed price plus the tip, so a base fee that doubles
    // between signing and inclusion still fits under the maximum.
    let max_fee = base.saturating_mul(2).saturating_add(priority);
    (priority as u128, max_fee as u128)
}
