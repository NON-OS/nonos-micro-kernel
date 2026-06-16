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

mod build;
mod checksum;
mod header;
mod iss;
mod parse;
pub mod rtt;
pub mod seq;
mod siphash;
mod state;
mod tcb;
pub mod window;

pub use build::{build, BuildRequest};
pub use header::{TcpHeader, FLAG_ACK, FLAG_FIN, FLAG_PSH, FLAG_RST, FLAG_SYN};
pub use iss::iss_for;
pub use parse::parse;
pub use siphash::siphash24;
pub use state::State;
pub use tcb::{Endpoint4, Tcb};

pub const MSS: usize = crate::protocol::SEGMENT_PAYLOAD_MAX;
pub const RWND_MAX: u16 = (crate::state::RX_DEPTH * MSS) as u16;

pub const MSL_MS: u64 = 30_000;

pub const RTO_MIN_MS: u32 = 200;
pub const RTO_MAX_MS: u32 = 60_000;
pub const RTO_INIT_MS: u32 = 1_000;

pub fn msl_2_ms() -> u64 {
    2 * MSL_MS
}
