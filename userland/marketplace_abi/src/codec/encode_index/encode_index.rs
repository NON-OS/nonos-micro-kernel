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

extern crate alloc;

use alloc::vec::Vec;

use super::encoded_index::EncodedIndex;
use crate::codec::encode_entry;
use crate::codec::writer::Writer;
use crate::types::MarketplaceIndex;

pub fn encode_index(index: &MarketplaceIndex) -> EncodedIndex {
    let mut signed: Vec<u8> = Vec::new();
    {
        let mut w = Writer::new(&mut signed);
        w.u32(index.schema_version);
        w.lp_string(&index.operator_id);
        w.fixed(&index.operator_pubkey);
        w.u64(index.published_at_ms);
        w.u64(index.serial);
        w.u32(index.entries.len() as u32);
        for entry in &index.entries {
            encode_entry::write(&mut w, entry);
        }
    }
    let mut blob = signed.clone();
    {
        let mut w = Writer::new(&mut blob);
        w.lp_bytes(&index.index_signature);
    }
    EncodedIndex { signed_bytes: signed, blob }
}
