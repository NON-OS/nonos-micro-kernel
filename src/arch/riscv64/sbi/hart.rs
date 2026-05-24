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

mod constants;
mod control;
mod status;
mod suspend;

pub use control::{hart_get_status, hart_start, hart_stop, hart_suspend};
pub use status::HartStatus;
pub use suspend::{
    suspend_non_retentive, suspend_retentive, SUSPEND_DEFAULT_NON_RETENTIVE,
    SUSPEND_DEFAULT_RETENTIVE,
};
