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

//! The claims this capsule tests against the live kernel.
//!
//! One read of the process table, not four, so the four verdicts describe the
//! same instant and cannot contradict each other because a capsule exited
//! between them.

use nonos_libc::mk_getpid;

use super::super::caps::{ADMIN, DEBUG, FILESYSTEM, MASK, RAW_HARDWARE};
use super::own_mask::own_mask_verdict;
use super::table::Tally;
use super::table_read::read;
use super::types::{Census, Check, Live, Verdict};

pub fn live() -> Option<Live> {
    let me = mk_getpid();
    let t: Tally = read(me)?;

    let mask = own_mask_verdict(me, t.own_mask);

    Some(Live {
        checks: [
            check(b"no running capsule can write a log", t.holders(DEBUG), t.total),
            admin(&t),
            check(b"every running capsule carries a capability mask", t.unmasked, t.total),
            Check {
                claim: b"this window holds exactly what its manifest declares",
                verdict: mask,
                found: u32::from(t.own_mask != MASK),
                of: 1,
            },
        ],
        census: Census {
            capsules: t.total,
            filesystem: t.holders(FILESYSTEM),
            raw_hardware: t.holders(RAW_HARDWARE),
            own_mask: t.own_mask,
        },
    })
}

// `found` counts what the claim forbids, so zero is the passing value for every
// check. A reader comparing them never has to know which way one runs.
fn check(claim: &'static [u8], found: u32, of: u32) -> Check {
    Check { claim, verdict: Verdict::from_bool(found == 0), found, of }
}

// init is the one process entitled to hold Admin, and it is excluded by name
// rather than by pid: the pid is an implementation detail, the name is what the
// manifest and the kernel agree on.
fn admin(t: &Tally) -> Check {
    check(b"only init holds admin authority", t.stray(ADMIN), t.total)
}
