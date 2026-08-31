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

use crate::wifi::{
    connect_network, driver_datapath, driver_stage, net_status, scan_adapters, scan_networks,
    DriverStage, ScanOutcome,
};

use super::edit_buffer::EditBuffer;
use super::state::{State, WifiConnect, WifiScan};

/// Ask the driver how far its radio came up and, only if it answered ready, scan.
/// The quick status probe is also a liveness gate: a scan is a long blocking call,
/// so it is only issued to a driver that just answered ready. A driver that
/// reported a hard failure shows its stage; one that did not answer the quick
/// probe at all is reported as unreachable rather than risking a long block on a
/// wedged driver. Every path leaves a visible result and keeps the cursor in
/// range.
pub fn run_wifi_scan(state: &mut State) {
    refresh_wifi_status(state);
    // Once connected, a channel scan would retune the radio off the live link and
    // drop the connection (and net_core's traffic), so refreshing the status is
    // all a connected panel does; the scan is only for finding networks to join.
    if state.wifi_connect == WifiConnect::Connected {
        return;
    }
    match state.wifi_stage {
        Some(DriverStage::Ready) => {
            let (count, outcome, stats) = scan_networks(&mut state.wifi_networks);
            // Strongest signal first, so the most reachable networks head the list
            // once the driver reports real RSSI.
            state.wifi_networks[..count].sort_unstable_by(|a, b| b.signal.cmp(&a.signal));
            state.wifi_network_count = count;
            state.wifi_scan = WifiScan::Done(outcome);
            state.wifi_stats = stats;
            if state.wifi_cursor >= count {
                state.wifi_cursor = count.saturating_sub(1);
            }
        }
        Some(_) => {
            // A hard bring-up failure; the stage line explains why there are none.
            state.wifi_network_count = 0;
            state.wifi_cursor = 0;
            state.wifi_scan = WifiScan::Idle;
        }
        None => {
            // The driver did not answer even the quick probe; say so rather than
            // issuing the long scan and freezing on an unresponsive driver.
            state.wifi_network_count = 0;
            state.wifi_cursor = 0;
            state.wifi_scan = WifiScan::Done(ScanOutcome::NoResponse);
        }
    }
}

/// Refresh the driver bring-up stage, the data-path frame counts and net_core's
/// lease without touching the radio, so the connected view (address, counters)
/// stays current on a live link that a channel scan would otherwise drop.
pub fn refresh_wifi_status(state: &mut State) {
    state.wifi_stage = driver_stage();
    state.wifi_datapath = driver_datapath();
    state.wifi_net = net_status();
}

/// Re-enumerate the wireless adapters into the WiFi panel state and keep the
/// selection cursor inside the new list.
pub fn refresh_wifi(state: &mut State) {
    state.wifi_adapter_count = scan_adapters(&mut state.wifi_adapters);
    if state.wifi_cursor >= state.wifi_adapter_count {
        state.wifi_cursor = state.wifi_adapter_count.saturating_sub(1);
    }
}

/// Begin or complete a connection to the selected network. A secured network
/// first opens the passphrase editor; the second call (or an open network on the
/// first) sends the driver the SSID and passphrase and runs the whole join, which
/// blocks for a few seconds. The result is recorded for the panel.
pub fn connect_selected(state: &mut State) {
    if state.wifi_network_count == 0 {
        return;
    }
    let idx = state.wifi_cursor.min(state.wifi_network_count - 1);
    let secured = state.wifi_networks[idx].secured;
    // A secured network needs a passphrase: open the editor on the first Enter.
    if secured && !state.wifi_pass_active {
        state.wifi_pass_active = true;
        state.wifi_pass = EditBuffer::empty();
        return;
    }
    // The passphrase is in (or the network is open): join now. The driver call
    // blocks for the length of the handshake, and the key handler returns Repaint
    // straight after, so the outcome is painted the same frame the join finishes.
    let idx = state.wifi_cursor.min(state.wifi_network_count - 1);
    let mut ssid = [0u8; 32];
    let slen = {
        let s = state.wifi_networks[idx].ssid();
        let n = s.len().min(32);
        ssid[..n].copy_from_slice(&s[..n]);
        n
    };
    let result = connect_network(&ssid[..slen], state.wifi_pass.as_slice());
    state.wifi_connect =
        if result.code == 0 { WifiConnect::Connected } else { WifiConnect::Failed(result) };
    state.wifi_pass_active = false;
}

/// Switch to the Wi-Fi tab and enumerate adapters. Does not scan here: a scan is a
/// blocking request to the driver, and running it on tab entry would freeze the
/// whole app if the driver were slow to answer. The user starts a scan with Enter,
/// which keeps the app responsive while navigating. Leaves editing behind, like
/// selecting any other section.
pub fn enter_wifi(state: &mut State) {
    state.editing = false;
    refresh_wifi(state);
    refresh_wifi_status(state);
}
