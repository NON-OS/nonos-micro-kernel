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

use alloc::vec::Vec;

use super::registry::WALLET_RAILS;

pub fn encode_wallet_rails() -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + WALLET_RAILS.len() * 40);
    out.extend_from_slice(&(WALLET_RAILS.len() as u32).to_le_bytes());
    for rail in WALLET_RAILS {
        out.push(rail.symbol.len() as u8);
        out.push(rail.family);
        out.extend_from_slice(&rail.status.to_le_bytes());
        out.extend_from_slice(&rail.flags.to_le_bytes());
        out.extend_from_slice(&rail.chain_id.to_le_bytes());
        out.extend_from_slice(&rail.contract);
        out.extend_from_slice(rail.symbol);
    }
    out
}
