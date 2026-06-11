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

mod api;
pub mod constants;
pub mod entry;
mod entry_frame;
mod entry_idt;
mod entry_types;
pub mod error;
mod handlers;
pub mod ops;
mod state;
pub mod table;

#[cfg(test)]
mod tests;

pub use api::verify_idt_integrity;
pub use constants::*;
pub use entry::PageFaultError;
pub use entry::{ExceptionHandler, ExceptionHandlerWithError, FnPtr, IdtEntry, InterruptFrame};
pub use error::IdtError;
pub use ops::{are_enabled, disable, disable_pic, enable, get_pic_masks, get_stats};
pub use ops::{
    get_vector_count, init, is_initialized, load_on_ap, register_handler, register_irq_handler,
};
pub use ops::{register_syscall_handler, remap_pic, set_pic_masks, unregister_irq_handler};
pub use ops::{without_interrupts, IdtStats};
pub use table::{Idt, IdtPtr};
