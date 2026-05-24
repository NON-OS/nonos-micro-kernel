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
mod config;
mod constants;
mod entry;
mod error;
mod registers;
mod state;

pub use config::{PmpAddressMode, PmpConfig};
pub use entry::PmpEntry;
pub use error::{PmpError, PmpResult};
pub use state::is_initialized;

pub fn init_pmp() -> PmpResult<()> {
    for index in 0..constants::PMP_ENTRY_COUNT {
        registers::clear_entry(index)?;
    }
    state::set_initialized();
    Ok(())
}

pub fn set_pmp_entry(index: usize, entry: &PmpEntry) -> PmpResult<()> {
    registers::write_entry(index, entry)
}

pub fn clear_pmp_entry(index: usize) -> PmpResult<()> {
    registers::clear_entry(index)
}
