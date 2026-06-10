// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

mod measurements;
mod pcr_ops;
mod quote_gen;
mod state;

pub use measurements::{
    set_bootloader_measurement, set_kernel_measurement, set_signature_attestation,
    set_zk_attestation,
};
pub use pcr_ops::{extend_pcr, extend_pcr_hash, get_boot_measurement};
pub use quote_gen::{
    generate_attestation_quote, generate_signed_quote_with_aik, verify_attestation_quote,
};
pub use state::{init_attestation, ATTESTATION_STATE};
