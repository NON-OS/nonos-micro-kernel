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
use spin::Mutex;

use super::admissible::admissible;
use super::clock;
use super::directory::{DirectoryMeta, ParsedDirectory};
use super::types::{Node, RouteError, TopologyError, NODE_CAP};

struct DirectoryState {
    meta: Option<DirectoryMeta>,
    nodes: Vec<Node>,
}

static STORE: Mutex<DirectoryState> = Mutex::new(DirectoryState { meta: None, nodes: Vec::new() });

pub fn replace(parsed: ParsedDirectory, now: u64) -> Result<(), TopologyError> {
    if parsed.nodes.is_empty() {
        return Err(TopologyError::Empty);
    }
    if parsed.nodes.len() > NODE_CAP || !fresh(parsed.meta, now) {
        return Err(TopologyError::BadTime);
    }
    let mut guard = STORE.lock();
    if guard.meta.map(|m| parsed.meta.epoch <= m.epoch).unwrap_or(false) {
        return Err(TopologyError::Stale);
    }
    *guard = DirectoryState { meta: Some(parsed.meta), nodes: parsed.nodes };
    Ok(())
}

pub fn snapshot() -> Result<Vec<Node>, RouteError> {
    let now = clock::now_ms().map_err(|_| RouteError::Expired)?;
    let guard = STORE.lock();
    let Some(meta) = guard.meta else {
        return Err(RouteError::Empty);
    };
    if !fresh(meta, now) {
        return Err(RouteError::Expired);
    }
    if !admissible(meta, crate::state::trusted_authority) {
        return Err(RouteError::Expired);
    }
    Ok(guard.nodes.clone())
}

pub fn meta() -> Option<DirectoryMeta> {
    STORE.lock().meta
}

fn fresh(meta: DirectoryMeta, now: u64) -> bool {
    now >= meta.not_before_ms && now < meta.not_after_ms
}
