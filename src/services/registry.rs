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

mod adopt;
mod auth;
mod endpoint;
mod error;
mod policy;
mod reserved;

pub(crate) use adopt::adopt_endpoint;
pub use endpoint::ServiceEndpoint;
pub use error::RegError;
pub use policy::required_caps;
pub(crate) use reserved::is_reserved_service;

pub const MAX_SERVICES: usize = 256;
pub(self) static ENDPOINTS: Mutex<Vec<ServiceEndpoint>> = Mutex::new(Vec::new());

/// Register a service endpoint on behalf of `pid`. This is the trusted core
/// path: the kernel spawn path calls it to publish a verified capsule's own
/// declared endpoint, so the only authorization it enforces is that `pid`
/// actually holds the capabilities the endpoint advertises. The runtime
/// `sys_service_register` syscall layers the caller-side register-right check
/// on top before reaching here (see `caller_has_register_right`).
pub fn register_endpoint(name: &str, port: u32, pid: u32, caps: u64) -> Result<(), RegError> {
    if !auth::owner_has_required(pid, caps) {
        return Err(RegError::PermissionDenied);
    }
    let mut eps = ENDPOINTS.lock();
    if eps.iter().any(|e| e.name == name && e.port == port && e.pid == pid) {
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

/// True if the current process is allowed to register a service name it does
/// not already own. Gates the runtime `sys_service_register` syscall so an
/// ordinary capsule cannot squat a peer's service; publishers granted
/// `RegisterService` (e.g. net.core) pass.
pub(crate) fn caller_has_register_right() -> bool {
    auth::caller_has_register_right()
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

/// Drop the endpoint registered under `name`, whoever owns it. A capsule's
/// reply endpoint is registered kernel-owned (pid 0), so it is not caught by
/// the per-pid sweep on teardown; without this an on-demand instance that
/// closes would leak its reply endpoint and the next spawn of the same slot
/// would collide on it. Returns true if an entry was removed.
pub fn unregister_endpoint_by_name(name: &str) -> bool {
    let mut eps = ENDPOINTS.lock();
    let before = eps.len();
    eps.retain(|e| e.name != name);
    before != eps.len()
}
