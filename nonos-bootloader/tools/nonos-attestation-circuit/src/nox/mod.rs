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

mod address;
mod check_work;
mod evidence;
mod fields;
mod hash_file;
mod hex32;
mod ids;
mod kind;
mod receipt_check;
mod record_evidence;
mod verifier;
mod write_json;

pub mod checks;

pub use address::{address_bytes, validate_address};
pub use check_work::check_work;
pub use evidence::work_evidence_hash;
pub use hash_file::hash_file;
pub use hex32::hex32;
pub use ids::{circuit_id, receipt_id, work_receipt_id};
pub use kind::WorkKind;
pub use receipt_check::check_receipt;
pub use record_evidence::record_evidence_hash;
pub use verifier::verifier_hash;
pub use write_json::write_json;
