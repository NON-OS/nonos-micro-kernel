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

//! Per-pid capability authority.
//!
//! The PCB's stored `Arc<CapabilityToken>` is the source of truth.
//! `caps_bits` is a derived bitmap cache so the cheap readers stay an
//! atomic load. Every mutator here mints a fresh token and updates
//! the cache under the same write lock through `install_token`.
//!
//! Token mint is fail-closed on the boot-session nonce: if the boot
//! singleton has not been initialized yet, `new_token` returns `None`
//! so no authority is silently bound to a zero nonce.

use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::capabilities::{bits_to_caps, caps_to_bits, CapabilityToken};
use crate::process::core::ProcessControlBlock;

use super::api::{with_process, with_process_mut};

static TOKEN_ID: AtomicU64 = AtomicU64::new(1);

pub(crate) fn new_token(pid: u32, bits: u64) -> Option<Arc<CapabilityToken>> {
    let boot_nonce = crate::security::boot_session::nonce()?;
    let asid = crate::memory::paging::manager::lookup_asid_for_process(pid).unwrap_or(0);
    let revocation_epoch =
        with_process(pid, |pcb| pcb.revocation_epoch.load(Ordering::Acquire)).unwrap_or(0);
    let mut tok = CapabilityToken {
        owner_module: pid as u64,
        permissions: bits_to_caps(bits),
        expires_at_ms: None,
        nonce: 0,
        signature: [0u8; 64],
        token_id: TOKEN_ID.fetch_add(1, Ordering::Relaxed),
        subject_capsule_id: pid,
        subject_asid: asid,
        subject_measurement: [0u8; 32],
        boot_session_nonce: boot_nonce,
        revocation_epoch,
        delegation_depth: 0,
    };
    crate::capabilities::token::sign_token(&mut tok).ok()?;
    Some(Arc::new(tok))
}

pub(crate) fn install_token(pcb: &ProcessControlBlock, new: Arc<CapabilityToken>) {
    let mut guard = pcb.capability_token.write();
    pcb.caps_bits.store(caps_to_bits(&new.permissions), Ordering::SeqCst);
    *guard = new;
}

/// Refresh the stored token after the PCB's address space has been
/// established. Constructors call this after `lifecycle::allocate` or
/// `lifecycle::inherit` so `subject_asid` reflects the real value
/// before the PCB is reachable through the process table.
pub(crate) fn rebind_address_space(pcb: &ProcessControlBlock) -> Option<()> {
    let bits = pcb.caps_bits.load(Ordering::Acquire);
    let fresh = new_token(pcb.pid, bits)?;
    install_token(pcb, fresh);
    Some(())
}

pub fn bits(pid: u32) -> Option<u64> {
    with_process(pid, |pcb| pcb.caps_bits.load(Ordering::Acquire))
}

/// Checks whether `pid` holds every bit in `mask`. Fail-closed on an
/// unknown pid.
pub fn has(pid: u32, mask: u64) -> bool {
    bits(pid).map(|b| (b & mask) == mask).unwrap_or(false)
}

/// Caller is responsible for any authority check before reaching here.
pub fn grant(pid: u32, mask: u64) -> Option<()> {
    with_process_mut(pid, |pcb| {
        let new_bits = pcb.caps_bits.load(Ordering::Acquire) | mask;
        let fresh = new_token(pcb.pid, new_bits)?;
        install_token(pcb, fresh);
        Some(())
    })
    .flatten()
}

pub fn revoke(pid: u32, mask: u64) -> Option<()> {
    with_process_mut(pid, |pcb| {
        pcb.revocation_epoch.fetch_add(1, Ordering::SeqCst);
        let new_bits = pcb.caps_bits.load(Ordering::Acquire) & !mask;
        let fresh = new_token(pcb.pid, new_bits)?;
        install_token(pcb, fresh);
        Some(())
    })
    .flatten()
}

/// One-shot manifest install. Replaces whatever inheritance-derived
/// token the PCB was born with. A second call is rejected so a stale
/// spawn path cannot re-issue authority.
pub fn install_spawn(pid: u32, mask: u64) -> Option<()> {
    with_process_mut(pid, |pcb| {
        let fresh = new_token(pcb.pid, mask)?;
        pcb.caps_manifest_installed
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .ok()
            .map(|_| install_token(pcb, fresh))
    })
    .flatten()
}
