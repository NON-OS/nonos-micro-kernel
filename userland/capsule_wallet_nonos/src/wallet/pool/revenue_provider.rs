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

use super::seam::Seam;

// The NOX token-utility surface: fee bps, cumulative protocol revenue, and the
// staking APR earned from fees. All values live-read from the contracts later.
pub trait RevenueProvider {
    fn fee_bps(&self) -> Seam<u32>;
    fn cumulative_revenue_wei(&self) -> Seam<[u8; 32]>;
    fn staking_apr_bps(&self) -> Seam<u32>;
    fn staked_wei(&self) -> Seam<[u8; 32]>;
}
