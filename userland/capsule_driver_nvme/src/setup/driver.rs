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

use crate::admin::{AdminQueue, ControllerIdentity, NamespaceIdentity, SmartHealth};
use crate::handles::BrokerHandles;
use crate::nvm::IoQueue;
use crate::regs::Regs;

pub struct Driver {
    pub _admin: AdminQueue,
    pub handles: BrokerHandles,
    pub regs: Regs,
    pub identity: ControllerIdentity,
    pub namespace: NamespaceIdentity,
    pub health: SmartHealth,
    pub io: Option<IoQueue>,
}
