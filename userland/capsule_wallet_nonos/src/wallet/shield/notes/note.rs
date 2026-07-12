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

// A shielded note: one entry in the local UTXO set. Secrets (spend_pk,
// blinding, nullifier) live only in the capsule and are encrypted at rest via
// the store's cipher seam; they never leave the device.
#[derive(Clone, Copy)]
pub struct Note {
    pub value: u128,
    pub asset_id: u32,
    pub spend_pk: [u8; 32],
    pub blinding: [u8; 32],
    pub leaf_index: u64,
    pub nullifier: [u8; 32],
    pub spent: bool,
}
