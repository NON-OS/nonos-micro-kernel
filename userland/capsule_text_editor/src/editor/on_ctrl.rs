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

use nonos_app_skeleton::EventOutcome;

use super::ctrl_copy::ctrl_copy;
use super::ctrl_paste::ctrl_paste;
use super::path_prompt;
use super::state::{PromptOp, State};

pub(super) fn on_ctrl(state: &mut State, code: u32) -> EventOutcome {
    match code {
        c if matches!(c, 0x43 | 0x63) => ctrl_copy(state),
        c if matches!(c, 0x4F | 0x6F) => path_prompt::start(state, PromptOp::Open),
        c if matches!(c, 0x53 | 0x73) => path_prompt::start(state, PromptOp::Save),
        c if matches!(c, 0x56 | 0x76) => ctrl_paste(state),
        _ => EventOutcome::Idle,
    }
}
