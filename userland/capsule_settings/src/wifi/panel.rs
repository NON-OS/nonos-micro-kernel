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

//! The WiFi settings panel: the list of detected networks, the selection
//! cursor, a passphrase editor for the chosen network, and the connection
//! status. Pure state so the renderer and the IPC layer are thin and the whole
//! flow (detect -> select -> enter key -> connect) is host-testable.

use alloc::vec::Vec;

use super::network::{ScanNetwork, SSID_MAX};
use super::wire::{encode_connect, parse_scan};

const MAX_NETWORKS: usize = 32;
/// WPA2-PSK passphrase bounds: 8 to 63 ASCII characters.
const PASSPHRASE_MIN: usize = 8;
const PASSPHRASE_MAX: usize = 63;

/// Where the connection stands, for the status line and to gate the editor.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WifiStatus {
    Idle,
    Scanning,
    /// Entering the passphrase for the selected secured network.
    Entering,
    Connecting,
    Connected,
    Failed,
}

pub struct WifiPanel {
    networks: [ScanNetwork; MAX_NETWORKS],
    count: usize,
    cursor: usize,
    pass: [u8; PASSPHRASE_MAX],
    pass_len: usize,
    status: WifiStatus,
}

impl Default for WifiPanel {
    fn default() -> Self {
        Self {
            networks: [ScanNetwork::default(); MAX_NETWORKS],
            count: 0,
            cursor: 0,
            pass: [0u8; PASSPHRASE_MAX],
            pass_len: 0,
            status: WifiStatus::Idle,
        }
    }
}

impl WifiPanel {
    /// Mark that a scan was requested; the service replies with results.
    pub fn begin_scan(&mut self) {
        self.status = WifiStatus::Scanning;
    }

    /// Load a scan-result buffer from the WiFi service, replacing the list.
    /// Duplicate SSIDs (the same network seen on several APs or channels) are
    /// merged, keeping the strongest signal, and the list is ordered strongest
    /// first so the display and the default selection are stable and useful.
    pub fn load_scan(&mut self, buf: &[u8]) {
        self.count = 0;
        let nets = &mut self.networks;
        let slot = &mut self.count;
        parse_scan(buf, |n| {
            if let Some(dup) = nets[..*slot].iter_mut().find(|e| e.ssid() == n.ssid()) {
                if n.signal > dup.signal {
                    dup.signal = n.signal;
                    dup.secured = n.secured;
                }
            } else if *slot < MAX_NETWORKS {
                nets[*slot] = n;
                *slot += 1;
            }
        });
        self.networks[..self.count].sort_unstable_by(|a, b| b.signal.cmp(&a.signal));
        if self.cursor >= self.count {
            self.cursor = self.count.saturating_sub(1);
        }
        self.status = WifiStatus::Idle;
    }

    pub fn networks(&self) -> &[ScanNetwork] {
        &self.networks[..self.count]
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn status(&self) -> WifiStatus {
        self.status
    }

    pub fn selected(&self) -> Option<&ScanNetwork> {
        self.networks[..self.count].get(self.cursor)
    }

    pub fn move_down(&mut self) {
        if self.cursor + 1 < self.count {
            self.cursor += 1;
        }
    }

    pub fn move_up(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    /// Choose the highlighted network. A secured network opens the passphrase
    /// editor; an open network is ready to connect immediately.
    pub fn choose(&mut self) {
        let secured = self.selected().map(|n| n.secured).unwrap_or(false);
        self.pass_len = 0;
        self.status = if secured { WifiStatus::Entering } else { WifiStatus::Idle };
    }

    pub fn push_pass(&mut self, ch: u8) {
        if self.status == WifiStatus::Entering && self.pass_len < PASSPHRASE_MAX {
            self.pass[self.pass_len] = ch;
            self.pass_len += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.status == WifiStatus::Entering {
            self.pass_len = self.pass_len.saturating_sub(1);
        }
    }

    /// The passphrase entered so far (for a masked-length display).
    pub fn passphrase_len(&self) -> usize {
        self.pass_len
    }

    /// Whether the entered passphrase is a valid WPA2-PSK length (8..=63). Open
    /// networks need none, so an empty passphrase is valid there. The UI uses
    /// this to enable the connect action.
    pub fn passphrase_valid(&self) -> bool {
        match self.selected() {
            Some(net) if net.secured => (PASSPHRASE_MIN..=PASSPHRASE_MAX).contains(&self.pass_len),
            _ => true,
        }
    }

    /// Build the connect request for the selected network, or `None` if a
    /// secured network's passphrase is not a valid WPA2 length. Moves to
    /// Connecting only when a request is produced.
    pub fn connect_request(&mut self) -> Option<Vec<u8>> {
        let net = self.selected()?;
        if net.secured && !(PASSPHRASE_MIN..=PASSPHRASE_MAX).contains(&self.pass_len) {
            return None;
        }
        let mut ssid = [0u8; SSID_MAX];
        let n = net.ssid().len();
        ssid[..n].copy_from_slice(net.ssid());
        let req = encode_connect(&ssid[..n], &self.pass[..self.pass_len]);
        self.status = WifiStatus::Connecting;
        Some(req)
    }

    /// Record the outcome the service reported.
    pub fn set_connected(&mut self, ok: bool) {
        self.status = if ok { WifiStatus::Connected } else { WifiStatus::Failed };
    }
}
