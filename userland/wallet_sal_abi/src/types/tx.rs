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

use crate::limits::{AMOUNT_BYTES, HASH_BYTES, MAX_PAYMENT_ID_LEN};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SalTxDraft {
    pub amount: [u8; AMOUNT_BYTES],
    pub fee: [u8; AMOUNT_BYTES],
    pub payment_id_len: u16,
    pub payment_id: [u8; MAX_PAYMENT_ID_LEN],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SalTxHash {
    pub bytes: [u8; HASH_BYTES],
}
