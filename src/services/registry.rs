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

extern crate alloc;
use alloc::vec::Vec;
use spin::Mutex;

mod endpoint;
mod error;

pub use endpoint::ServiceEndpoint;
pub use error::RegError;

pub const MAX_SERVICES: usize = 256;
static ENDPOINTS: Mutex<Vec<ServiceEndpoint>> = Mutex::new(Vec::new());

fn caller_can_register() -> bool {
    match crate::process::current_pid() {
        None => true,
        Some(pid) if pid <= 64 => true,
        Some(_) => {
            let token = crate::syscall::capabilities::current_caps_or_default();
            token.can_register_service() || token.is_admin()
        }
    }
}

pub fn register_endpoint(name: &str, port: u32, pid: u32, caps: u64) -> Result<(), RegError> {
    if !caller_can_register() {
        return Err(RegError::PermissionDenied);
    }
    let mut eps = ENDPOINTS.lock();
    if eps
        .iter()
        .any(|e| e.name == name && e.port == port && e.pid == pid)
    {
        return Ok(());
    }
    if eps.iter().any(|e| e.name == name || e.port == port) {
        return Err(RegError::Exists);
    }
    if eps.len() >= MAX_SERVICES {
        return Err(RegError::Full);
    }
    eps.push(ServiceEndpoint::new(name, port, pid, caps));
    Ok(())
}

pub fn lookup_service(name: &str) -> Option<ServiceEndpoint> {
    ENDPOINTS.lock().iter().find(|e| e.name == name).cloned()
}

pub fn lookup_port(port: u32) -> Option<ServiceEndpoint> {
    ENDPOINTS.lock().iter().find(|e| e.port == port).cloned()
}

pub fn unregister_endpoints_for_pid(pid: u32) -> usize {
    let mut eps = ENDPOINTS.lock();
    let before = eps.len();
    eps.retain(|e| e.pid != pid);
    before - eps.len()
}
