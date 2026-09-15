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

//! The kernel module's own shape, minus `derive`, which needs the CRB
//! transport and the kernel RNG. Everything it calls is here.

#[path = "../../../../../../src/security/tpm/machine_key/consts.rs"]
pub mod consts;
#[path = "../../../../../../src/security/tpm/machine_key/create.rs"]
pub mod create;
#[path = "../../../../../../src/security/tpm/machine_key/error.rs"]
pub mod error;
#[path = "../../../../../../src/security/tpm/machine_key/flush.rs"]
pub mod flush;
#[path = "../../../../../../src/security/tpm/machine_key/hmac.rs"]
pub mod hmac;
#[path = "../../../../../../src/security/tpm/machine_key/pcrs.rs"]
pub mod pcrs;
#[path = "../../../../../../src/security/tpm/machine_key/policy.rs"]
pub mod policy;
#[path = "../../../../../../src/security/tpm/machine_key/session.rs"]
pub mod session;
#[path = "../../../../../../src/security/tpm/machine_key/wire.rs"]
pub mod wire;

pub use error::KeyError;
pub use pcrs::BOUND_PCRS;

#[cfg(test)]
mod command_tests;
#[cfg(test)]
mod live_tests;
#[cfg(test)]
mod parse_tests;
#[cfg(test)]
mod swtpm;
#[cfg(test)]
mod swtpm_io;
#[cfg(test)]
mod wire_tests;
