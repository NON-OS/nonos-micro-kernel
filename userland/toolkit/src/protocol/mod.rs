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

pub mod errno;
pub mod header;
pub mod ops;

pub use errno::{E_BAD_OP, E_INVAL, E_SHORT, E_SURFACE, STATUS_OK};
pub use header::{decode, encode, Header, HDR_LEN, MAGIC};
pub use ops::{
    IPC_PAYLOAD_MAX, THEME_PAYLOAD_LEN, TOOLKIT_ENDPOINT, TOOLKIT_OP_ANIMATION_TICK,
    TOOLKIT_OP_COMPONENT_RENDER, TOOLKIT_OP_HEALTHCHECK, TOOLKIT_OP_THEME_APPLY,
    TOOLKIT_OP_THEME_GET,
};
