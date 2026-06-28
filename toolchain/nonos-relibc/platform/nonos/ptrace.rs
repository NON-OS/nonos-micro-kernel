use super::{
    super::{PalPtrace, types::*},
    Sys,
};
use crate::{
    error::{Errno, Result},
    header::errno::ENOSYS,
};

impl PalPtrace for Sys {
    unsafe fn ptrace(_request: c_int, _pid: pid_t, _addr: *mut c_void, _data: *mut c_void) -> Result<c_int> { Err(Errno(ENOSYS)) }
}
