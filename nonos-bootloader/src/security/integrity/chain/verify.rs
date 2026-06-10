// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::hash::{chain_hash, constant_time_eq_32};
use crate::security::integrity::types::ChainLink;

pub fn verify_chain_links(links: &[ChainLink], count: usize) -> bool {
    let mut prev = [0u8; 32];
    for i in 0..count {
        let link = &links[i];
        let expected = chain_hash(&prev, &link.measurement, link.stage as u8);
        if !constant_time_eq_32(&expected, &link.cumulative) {
            return false;
        }
        prev = link.cumulative;
    }
    true
}
