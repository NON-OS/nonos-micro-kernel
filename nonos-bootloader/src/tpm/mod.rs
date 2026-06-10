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

pub mod core;
pub mod crypto;
pub mod hardware;
pub mod security;
pub mod storage;
pub mod types;

pub use core::{initialize_tpm, TmpDevice, TmpError, TmpResult};
pub use crypto::{compute_hash, extend_pcr, get_random, read_pcr};
pub use hardware::{acquire_locality, release_locality, send_command};
pub use security::{create_attestation, create_session, verify_quote};
pub use storage::{create_key, load_key, nv_read, nv_write};
pub use types::{NvIndex, PcrBank, Quote, Session, TmpHandle};
