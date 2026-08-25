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

/// True once the server's encrypted flight has arrived in full: at least three
/// application-data records (EncryptedExtensions, the certificate material, and
/// the server Finished at the minimum, more when the chain is large) AND a clean
/// record boundary at the end.
///
/// The boundary requirement is the whole point. Stopping the instant a third
/// record was counted returned the entire read buffer, which usually still held
/// the front of a fourth record mid-arrival; the parser downstream then read
/// that fragment as a truncated record and failed the handshake. Walking every
/// record to the end and refusing while any body is still incomplete hands back
/// a flight that always ends exactly on a record boundary.
pub fn server_finished_flight_ready(bytes: &[u8]) -> bool {
    let mut pos = 0usize;
    let mut app_records = 0usize;
    while pos + 5 <= bytes.len() {
        let len = u16::from_be_bytes([bytes[pos + 3], bytes[pos + 4]]) as usize;
        let end = pos + 5 + len;
        if end > bytes.len() {
            // A record body is still arriving. Not ready, keep reading.
            return false;
        }
        if bytes[pos] == 23 {
            app_records += 1;
        }
        pos = end;
    }
    // Clean boundary reached with the whole encrypted flight in hand.
    pos == bytes.len() && app_records >= 3
}
