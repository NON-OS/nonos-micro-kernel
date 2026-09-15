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

//! A precision touchpad: its report descriptor and the frames it sends.

mod finger;
mod frame;
mod mode;
mod report_desc;

pub use frame::{touch_report, Touch};
pub use report_desc::{
    report_descriptor, MODE_REPORT_ID, TOUCH_REPORT_ID, TOUCH_REPORT_LEN, X_MAX, Y_MAX,
};
