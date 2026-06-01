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

use super::fields::{ReceiptInput, SignedReceipt};

pub fn build_record(f: &ReceiptInput, r: &SignedReceipt) -> Vec<u8> {
    let mut out = Vec::with_capacity(297);
    out.extend_from_slice(&r.user);
    out.extend_from_slice(&f.capsule_id);
    out.extend_from_slice(&f.publisher);
    out.extend_from_slice(&f.amount);
    out.extend_from_slice(&f.nonce);
    out.extend_from_slice(&f.epoch);
    out.extend_from_slice(&f.expiry);
    out.extend_from_slice(&f.receipt_type);
    out.extend_from_slice(&r.signature);
    out
}
