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

use super::hash::{chain_hash, compute_measurement};
use super::verify::verify_chain_links;
use crate::security::integrity::types::{BootStage, ChainLink};

const MAX_LINKS: usize = 16;

pub struct IntegrityChain {
    links: [ChainLink; MAX_LINKS],
    count: usize,
    sealed: bool,
}

impl IntegrityChain {
    pub const fn new() -> Self {
        Self { links: [ChainLink::empty(); MAX_LINKS], count: 0, sealed: false }
    }

    pub fn extend(&mut self, stage: BootStage, data: &[u8], ts: u64) -> Option<[u8; 32]> {
        if self.sealed || self.count >= MAX_LINKS {
            return None;
        }
        let m = compute_measurement(data);
        let p = if self.count > 0 { self.links[self.count - 1].cumulative } else { [0u8; 32] };
        let c = chain_hash(&p, &m, stage as u8);
        self.links[self.count] = ChainLink { stage, measurement: m, cumulative: c, timestamp: ts };
        self.count += 1;
        Some(c)
    }

    pub fn seal(&mut self) {
        self.sealed = true;
    }
    pub fn get_final_hash(&self) -> Option<[u8; 32]> {
        if self.count == 0 {
            None
        } else {
            Some(self.links[self.count - 1].cumulative)
        }
    }
    pub fn verify_chain(&self) -> bool {
        verify_chain_links(&self.links, self.count)
    }
}
