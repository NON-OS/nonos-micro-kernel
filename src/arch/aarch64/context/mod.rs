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

mod capture;
mod enter;
mod kernel;
mod restored;
mod resume;
mod setup;
mod switch;
mod types;

pub use capture::save_user_frame;
pub use enter::{enter_user, EnterError, SPSR_EL0T_INITIAL};
pub use kernel::Context;
pub use restored::set_restored_flag;
pub use resume::{resume_user, ResumeError};
pub use setup::{setup_initial_user_pcb_aarch64, SetupError};
pub(crate) use switch::switch_to_user_pcb_aarch64;
pub use types::{SavedUser, UserEntry};
