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

use crate::settings::ipc::IpcError;
use crate::settings::state::status::StatusKind;
use crate::settings::state::State;

pub fn report(state: &mut State, err: IpcError) {
    let msg: &[u8] = match err {
        IpcError::NotFound => b"policy service not registered",
        IpcError::SendFailed => b"ipc send failed",
        IpcError::RecvTimeout => b"policy timeout",
        IpcError::ShortReply => b"short reply",
        IpcError::BadHeader => b"bad header",
        IpcError::KindMismatch => b"kind mismatch",
        IpcError::Status(_) => b"policy rejected",
    };
    state.status.set(StatusKind::Error, msg);
}
