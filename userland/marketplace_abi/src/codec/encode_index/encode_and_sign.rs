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

use super::encode_index::encode_index;
use crate::codec::writer::Writer;
use crate::types::MarketplaceIndex;

pub fn encode_and_sign<F>(mut index: MarketplaceIndex, sign: F) -> (Vec<u8>, [u8; 64])
where
    F: FnOnce(&[u8]) -> [u8; 64],
{
    index.index_signature.clear();
    let encoded = encode_index(&index);
    let signature = sign(&encoded.signed_bytes);
    let mut blob = encoded.signed_bytes;
    {
        let mut w = Writer::new(&mut blob);
        w.lp_bytes(&signature);
    }
    (blob, signature)
}
