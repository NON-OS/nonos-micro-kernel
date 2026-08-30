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

//! Why a dimmed control is dimmed. A disabled row or cell reports the model gap
//! that blocks it instead of a blanket "not implemented", so the status bar
//! names the missing piece rather than the missing handler.

pub(in crate::editor) const NO_BLOCK_MODEL: &[u8] =
    b"unavailable: alignment, lists and tables need block-level styling";
pub(in crate::editor) const NO_HANDLER: &[u8] = b"unavailable: not built into this capsule";
pub(in crate::editor) const NO_DOC_MODE: &[u8] = b"unavailable: tables need document mode";

pub(in crate::editor) const NO_TABLE_AT_CARET: &[u8] = b"place the caret inside a table first";
