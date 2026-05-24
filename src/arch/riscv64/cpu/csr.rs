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

mod bits;
mod constants;
mod error;
mod read;
mod status;
mod write;

pub use bits::{clear_csr, set_csr};
pub use constants::*;
pub use error::{CsrError, CsrResult};
pub use read::{read_csr, read_cycle, read_scause, read_sepc, read_sstatus, read_stval, read_time};
pub use status::*;
pub use write::write_csr;
