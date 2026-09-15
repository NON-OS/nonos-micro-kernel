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

//! The six commands, in order, with every slot given back on every path.
//!
//! A TPM has very few transient slots, three on some parts. The attestation
//! key holds one for the life of the boot. This sequence takes two more and
//! must return both whether or not the HMAC succeeded, or the second
//! derivation on the machine fails with `TPM_RC_OBJECT_MEMORY` and looks like
//! a policy failure to anyone who did not read the code.

use spin::Mutex;

use super::consts::{DIGEST_LEN, LABEL_MAX, NONCE_LEN};
use super::create::{build_create, parse_create};
use super::error::KeyError;
use super::flush::build_flush;
use super::hmac::{build_hmac, parse_hmac};
use super::pcrs::BOUND_PCRS;
use super::policy::{build_get_digest, build_policy_pcr, parse_get_digest, parse_policy_pcr};
use super::session::{build_start, parse_start};
use super::run::run;
use crate::crypto::fill_random_bytes;

/// One derivation at a time. Two in flight would each hold a session and an
/// object, and that is the whole slot budget of a small part.
static IN_PROGRESS: Mutex<()> = Mutex::new(());

/// The key for `label` on this machine in this boot state, or why not.
pub fn derive(label: &[u8]) -> Result<[u8; DIGEST_LEN], KeyError> {
    if label.is_empty() || label.len() > LABEL_MAX {
        return Err(KeyError::BadLabel);
    }
    let _one = IN_PROGRESS.lock();
    let mut nonce = [0u8; NONCE_LEN];
    fill_random_bytes(&mut nonce);
    let session = parse_start(&run(&build_start(&nonce))?)?;
    let result = under_session(session, label);
    let _ = run(&build_flush(session));
    result
}

fn under_session(session: u32, label: &[u8]) -> Result<[u8; DIGEST_LEN], KeyError> {
    parse_policy_pcr(&run(&build_policy_pcr(session, &BOUND_PCRS))?)?;
    let policy = parse_get_digest(&run(&build_get_digest(session))?)?;
    let key = parse_create(&run(&build_create(&policy))?)?;
    let result = run(&build_hmac(key, session, label)).and_then(|r| parse_hmac(&r));
    let _ = run(&build_flush(key));
    result
}
