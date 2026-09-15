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

//! A key this machine can derive and no other machine can.
//!
//! Nothing is stored. The key is `TPM2_HMAC` over a caller's label under a
//! primary object the TPM derives from its owner seed and a template whose
//! authorisation policy is the current value of the boot PCRs. Two things
//! follow. The same part in the same boot state answers with the same key
//! every time, which is what lets an amnesic system reopen a volume it
//! encrypted last week. And a different boot state answers with a different
//! key twice over: the policy digest changes the template and so the
//! derivation, and the policy session that has to authorise the HMAC fails
//! before the TPM would use the key at all.
//!
//! Sealing a blob would do the same job and leave a file to lose. This leaves
//! nothing. Clearing the owner hierarchy changes the seed and every key with
//! it, which is the documented way to make the data unrecoverable on purpose.

mod consts;
mod create;
mod derive;
mod error;
mod flush;
mod hmac;
mod pcrs;
mod policy;
mod run;
mod session;
mod wire;

pub use consts::LABEL_MAX;
pub use derive::derive;
pub use error::KeyError;
pub use pcrs::BOUND_PCRS;
