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
pub mod cc;
mod checksum;
mod constants;
mod header;
mod iss;
mod msl_2_ms;
mod parse;
pub mod rtt;
pub mod seq;
mod siphash;
mod state;
mod tcb;
pub mod window;

pub use build::{build, BuildRequest};
pub use constants::{
    DUP_ACK_THRESH, INIT_CWND, MAX_CONN_PER_PID, MAX_RETX, MSL_MS, MSS, REASM_MAX_SEGS, RTO_INIT_MS,
    RTO_MAX_MS, RTO_MIN_MS, RWND_MAX, SND_BUF_MAX,
};
pub use header::{TcpHeader, FLAG_ACK, FLAG_FIN, FLAG_PSH, FLAG_RST, FLAG_SYN};
pub use iss::iss_for;
pub use msl_2_ms::msl_2_ms;
pub use parse::parse;
pub use siphash::siphash24;
pub use state::State;
pub use tcb::{Endpoint4, Tcb};
