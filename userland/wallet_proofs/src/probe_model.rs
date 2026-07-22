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

fn advance(step: u8) -> u8 {
    (step + 1) % STEPS
}

/// app.rs: probe only with an account, after the user has paused, on a probe tick.
fn probes(address_ready: bool, ticks: u32, last_active: u32) -> bool {
    address_ready && ticks.wrapping_sub(last_active) >= IDLE_GATE && ticks % 2 == 0
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
fn no_probe_while_interacting() {
    // last_active == ticks means the user just acted: never probe on that tick.
    for t in 0..64u32 {
        assert!(!probes(true, t, t), "no network while the hand is on the wallet");
    }
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
