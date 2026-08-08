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

mod address_space;
pub mod api;
mod core;
mod faults;
mod mapping;
mod protection;
mod query;
pub mod shootdown;
mod tlb_scope;
mod translation;

pub use self::core::PagingManager;
pub use api::*;
pub use shootdown::{
    flush_tlb_all_smp, flush_tlb_one_smp, flush_tlb_range_smp, handle_shootdown_ipi, ASID_KERNEL,
};
