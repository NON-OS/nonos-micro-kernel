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

//! The refresh loop can never freeze the UI. This models the step machine in
//! `wallet/event/probe_tick.rs`, the idle gate in `wallet/app.rs`, and the
//! per-operation socket timeouts in `wallet/net/*.rs`, and checks the
//! invariants that keep the wallet responsive. Change a constant in the wallet
//! and the matching constant here should change with it.
//!
//! Several checks assert a bound on these mirrored constants, which is the whole
//! point of the proof, so the constant-assertion lint is not useful here.
#![allow(clippy::assertions_on_constants)]
// Mirror the wallet's own `ticks % 2 == 0` phrasing rather than a lint-preferred
// rewrite, so this model reads the same as the code it models.
#![allow(clippy::manual_is_multiple_of)]

const STEPS: u8 = 8; // probe_tick.rs
const IDLE_GATE: u32 = 3; // app.rs: skip probing for this many ticks after input

// net/*.rs mk_ipc_call_timeout bounds, in milliseconds.
const T_DNS: u64 = 1200;
const T_OPEN: u64 = 500;
const T_CONNECT: u64 = 1800;
const T_SEND: u64 = 800;
const T_RECV: u64 = 80;

// Non-socket IPC bounds: the TLS handshake's crypto-capsule calls
// (tls13/crypto_status.rs, tls13/hash_sha384.rs), the keyring call
// (ipc/call.rs), and the service health probe (net/health.rs). The Send
// freeze reproduced through these, not the sockets: any blocking IPC on the
// UI thread must be bounded, whoever the peer is.
const T_CRYPTO: u64 = 1500;
const T_KEYRING: u64 = 4000;
const T_HEALTH: u64 = 800;

fn advance(step: u8) -> u8 {
    (step + 1) % STEPS
}

/// app.rs: probe only when the user has paused (idle). Mouse movement does not
/// count as activity (see app.rs on_event), so it never defers the probe; a real
/// click or keypress does, keeping the UI instant under the hand and never
/// freezing it on a blocking read mid-interaction.
fn probes(address_ready: bool, ticks: u32, last_active: u32) -> bool {
    if !address_ready {
        return false;
    }
    ticks.wrapping_sub(last_active) >= IDLE_GATE && ticks % 2 == 0
}

/// Worst-case wall time of one fetch if every op times out (a blackholed peer):
/// DNS, open, connect, send, read, send, read. Finite by construction.
fn worst_case_fetch_ms() -> u64 {
    T_DNS + T_OPEN + T_CONNECT + T_SEND + T_RECV + T_SEND + T_RECV
}

#[test]
fn cycle_covers_every_field_once() {
    let mut seen = [false; STEPS as usize];
    let mut s = 0u8;
    for _ in 0..STEPS {
        seen[s as usize] = true;
        s = advance(s);
    }
    assert!(seen.iter().all(|&b| b), "every field refreshed once per cycle");
    assert_eq!(s, 0, "cycle wraps back to the start");
}

#[test]
fn one_step_per_tick_never_a_burst() {
    assert_eq!(advance(3), 4);
    assert_eq!(advance(7), 0);
}

#[test]
fn active_interaction_never_freezes_the_ui() {
    // A click or keypress every tick (last_active == ticks) defers the probe
    // every time, so a blocking read never runs under the user's hand. Movement
    // is handled separately (app.rs does not treat it as activity), so it never
    // reaches this gate: the fee and staking reads still land once the user
    // pauses, which probing_resumes_once_idle covers.
    for t in 0..64u32 {
        assert!(!probes(true, t, t), "no blocking probe while actively interacting");
    }
}

#[test]
fn no_probe_right_after_a_click() {
    // The tick right after an odd-tick click (not a forced tick) does not probe,
    // so a click is serviced without waiting on the network.
    assert!(!probes(true, 5, 5), "no probe on the tick of a click");
    assert!(!probes(true, 7, 7), "no probe immediately after a click");
}

#[test]
fn probing_resumes_once_idle() {
    assert!((IDLE_GATE..IDLE_GATE + 8).any(|t| probes(true, t, 0)), "auto-refresh still runs");
}

#[test]
fn no_probe_before_an_account_exists() {
    assert!(!(0..64u32).any(|t| probes(false, t, 0)));
}

#[test]
fn worst_case_is_bounded_and_recv_is_snappy() {
    let wc = worst_case_fetch_ms();
    assert!(wc > 0 && wc < 5500, "a dead peer costs a finite {wc} ms, never the UI");
    assert!(T_RECV <= 100, "the polled recv stays responsive");
}

#[test]
fn every_ui_thread_ipc_is_time_bounded() {
    // The invariant behind the Send-path freeze fix: no IPC from the UI
    // thread may wait forever. Each bound is finite and small enough that
    // even a fully wedged peer returns control within seconds.
    for (name, bound) in [
        ("crypto capsule (TLS handshake)", T_CRYPTO),
        ("keyring", T_KEYRING),
        ("health probe", T_HEALTH),
    ] {
        assert!(bound > 0 && bound <= 5000, "{name} bound {bound} ms must be finite and short");
    }
    // The keyring bound covers its slowest real op: PBKDF2 with 2048 rounds
    // plus BIP32 derivation during HD generate/recover.
    assert!(T_KEYRING >= 2000, "keyring bound must leave room for PBKDF2-2048");
}
