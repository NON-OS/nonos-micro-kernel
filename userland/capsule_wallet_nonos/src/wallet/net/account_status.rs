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

#[derive(Clone, Copy)]
pub struct AccountStatus {
    pub balance_ready: bool,
    pub balance_wei: [u8; 32],
    pub nonce_ready: bool,
    pub nonce: u64,
    pub fee_ready: bool,
    pub fee_wei: u64,
}

impl AccountStatus {
    pub fn empty() -> Self {
        Self { balance_ready: false, balance_wei: [0; 32], nonce_ready: false, nonce: 0, fee_ready: false, fee_wei: 0 }
    }
}
