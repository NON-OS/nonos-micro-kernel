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

use spin::Mutex;

use super::state::IntegrityChain;
use crate::security::integrity::types::BootStage;

pub static INTEGRITY_CHAIN: Mutex<IntegrityChain> = Mutex::new(IntegrityChain::new());

pub fn record_stage(stage: BootStage, data: &[u8], timestamp: u64) -> Option<[u8; 32]> {
    let mut chain = INTEGRITY_CHAIN.lock();
    chain.extend(stage, data, timestamp)
}

pub fn get_boot_integrity_hash() -> Option<[u8; 32]> {
    let chain = INTEGRITY_CHAIN.lock();
    chain.get_final_hash()
}

pub fn seal_chain() {
    let mut chain = INTEGRITY_CHAIN.lock();
    chain.seal();
}

pub fn verify_integrity() -> bool {
    let chain = INTEGRITY_CHAIN.lock();
    chain.verify_chain()
}
