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

//! The serving stage. Once the firmware is running and the MAC is initialised,
//! this builds the radio and answers requests forever. Two request families
//! arrive on the one service inbox: net_core's link operations, handled by the
//! shared `netif::serve` against the `RtlLink`, and a Wi-Fi control family, handled
//! by [`control`], that scans and joins. The loop also advances a background
//! scanner between requests so a scan is answered from a cache rather than a sweep.
//!
//! The stage is split by concern: [`stage`] is the bring-up result, [`radio`]
//! builds and holds the radio, [`scanner`] keeps the network picture current,
//! [`control`] is the control protocol, and [`connect`] runs a WPA2 join.

mod connect;
mod control;
mod radio;
mod scanner;
mod stage;

pub use stage::Stage;

use nonos_libc::{mk_ipc_recv, mk_ipc_send};
use nonos_wifi_core::netif::{self, wire, MAX_RESPONSE};

use crate::link::DeadLink;
use crate::setup::Mapped;
use crate::status;

use connect::Session;
use control::control;
use radio::{build_radio, Radio};
use scanner::Scanner;

/// The kernel endpoint the driver sends replies to; the spawn's reply inbox routes
/// them back to whoever asked.
const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_0018;
/// The driver's own service inbox, where requests arrive.
const SERVICE_INBOX: u64 = 0;
/// How long the serve loop waits for a request before taking an idle scan step.
/// Short enough that a background sweep makes steady progress, long enough that the
/// loop is not busy-waiting.
const IDLE_TIMEOUT_MS: u64 = 100;

/// The 2.4GHz channels the background scan and the pre-join beacon hunt hop through.
const SCAN_CHANNELS: [u8; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
/// The scratch buffer a scan or a beacon hunt reads each frame into.
const SCAN_FRAME_MAX: usize = 2048;
/// The channel the radio parks on until a scan retunes it. Channel 1 is always
/// legal and is where a passive scan starts.
const DEFAULT_CHANNEL: u8 = 1;

/// Serve forever. Takes the mapped chip if bring-up got that far, and the stage it
/// reached. This never exits and never hangs: even a failed bring-up keeps the
/// service registered and answers the panel with the stage, which is the only
/// diagnostic a machine without a serial port has.
pub fn run(mapped: Option<Mapped>, stage: Stage) -> ! {
    let (mut radio, stage) = build_radio(mapped, stage);
    let mut session: Option<Session> = None;
    let mut scanner = Scanner::new();
    status::line(b"[rtl8821ce] serving net_core\n");

    let mut rx = [0u8; wire::HDR_LEN + wire::MAX_FRAME];
    let mut tx = [0u8; MAX_RESPONSE];
    loop {
        let n = mk_ipc_recv(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), IDLE_TIMEOUT_MS);
        if n > 0 {
            let req = &rx[..n as usize];
            // net_core's link operations first; the control family is the
            // fallthrough. A down radio answers through a link that is always down.
            let served = match &mut radio {
                Radio::Up(up) => netif::serve(req, &mut up.link, &mut tx),
                Radio::Down => netif::serve(req, &mut DeadLink, &mut tx),
            };
            if let Some(len) = served {
                let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), len);
            } else if let Some(len) =
                control(req, &mut radio, &mut session, &scanner, stage, &mut tx)
            {
                let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), len);
            }
        }
        // Advance the background scan while up and not associated. Draining runs on
        // every pass so net_core traffic cannot starve it; the channel only hops on
        // an idle pass, where the receive timeout genuinely elapsed, so the dwell is
        // real time and never knocks a live connection off its channel.
        if session.is_none() {
            if let Radio::Up(up) = &mut radio {
                scanner.drain(&mut up.link, &up.regs);
                if n <= 0 {
                    scanner.advance(&up.regs);
                }
            }
        }
    }
}
