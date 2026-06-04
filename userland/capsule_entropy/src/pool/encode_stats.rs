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
use super::types::Stats;

pub fn encode_stats(s: &Stats) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[0..8].copy_from_slice(&s.uptime_requests.to_le_bytes());
    out[8..16].copy_from_slice(&s.bytes_served.to_le_bytes());
    out[16..24].copy_from_slice(&s.last_reseed_request.to_le_bytes());
    out[24..32].copy_from_slice(&s.source_failures.to_le_bytes());
    out
}
