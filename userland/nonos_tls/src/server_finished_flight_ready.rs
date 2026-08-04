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

pub fn server_finished_flight_ready(bytes: &[u8]) -> bool {
    let mut pos = 0usize;
    let mut app_records = 0usize;
    while pos + 5 <= bytes.len() {
        let len = u16::from_be_bytes([bytes[pos + 3], bytes[pos + 4]]) as usize;
        let end = pos + 5 + len;
        if end > bytes.len() {
            return false;
        }
        if bytes[pos] == 23 {
            app_records += 1;
        }
        if app_records >= 3 {
            return true;
        }
        pos = end;
    }
    false
}
