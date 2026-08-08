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
use nonos_libc::mk_time_millis;
use spin::Mutex;

use super::surb_types::{normalize_ttl, Surb, CAP, DEFAULT_TTL_MS};
use super::{surb_id, surb_tag};

static SURBS: Mutex<Vec<Surb>> = Mutex::new(Vec::new());

pub fn create(owner: u32, session: u32, cred: &[u8; 32], ttl_ms: u64) -> Option<(u32, [u8; 32])> {
    let now = now_ms()?;
    let mut g = SURBS.lock();
    prune(&mut g, now);
    let (id, seed) = surb_id::create(&g, owner)?;
    let tag = surb_tag::make(owner, session, id, cred, &seed)?;
    let expires_ms = now.saturating_add(normalize_ttl(ttl_ms));
    if g.len() >= CAP {
        g.remove(0);
    }
    g.push(Surb { owner, id, session, tag, expires_ms });
    Some((id, tag))
}

pub fn consume(owner: u32, id: u32, tag: &[u8; 32]) -> Option<u32> {
    let now = now_ms()?;
    let mut g = SURBS.lock();
    prune(&mut g, now);
    let pos =
        g.iter().position(|s| s.owner == owner && s.id == id && surb_tag::matches(&s.tag, tag))?;
    Some(g.remove(pos).session)
}

pub fn default_ttl_ms() -> u64 {
    DEFAULT_TTL_MS
}

fn now_ms() -> Option<u64> {
    let now = mk_time_millis();
    (now >= 0).then_some(now as u64)
}

fn prune(g: &mut Vec<Surb>, now: u64) {
    g.retain(|s| s.expires_ms > now);
}
