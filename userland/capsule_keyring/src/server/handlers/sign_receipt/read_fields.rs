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
use crate::server::eip712::ReceiptFields;
use crate::server::field32::field32;

pub fn read_fields(p: &[u8], user: [u8; 20]) -> ReceiptFields {
    let mut publisher = [0u8; 20];
    publisher.copy_from_slice(&p[40..60]);
    ReceiptFields {
        capsule_id: field32(p, 8),
        user,
        publisher,
        amount_nox: field32(p, 60),
        nonce: field32(p, 92),
        epoch: field32(p, 124),
        expiry: field32(p, 156),
        receipt_type: field32(p, 188),
    }
}
