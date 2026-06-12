// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

mod embed;
mod spawn;
mod state;

pub use spawn::{spawn_driver_i2c_pci_capsule, SpawnError};
pub use state::shared_state;
