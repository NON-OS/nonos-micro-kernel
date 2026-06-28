use super::{
    super::{PalSignal, types::*},
    Sys,
};
#[allow(deprecated)]
use crate::header::sys_time::itimerval;
use crate::{
    error::{Errno, Result},
    header::{
        bits_sigset_t::sigset_t,
        errno::ENOSYS,
        signal::{sigaction, siginfo_t, sigval, stack_t},
        time::timespec,
    },
};

impl PalSignal for Sys {
    #[allow(deprecated)]
    fn getitimer(_which: c_int, _out: &mut itimerval) -> Result<()> { Err(Errno(ENOSYS)) }
    fn kill(_pid: pid_t, _sig: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn sigqueue(_pid: pid_t, _sig: c_int, _val: sigval) -> Result<()> { Err(Errno(ENOSYS)) }
    fn killpg(_pgrp: pid_t, _sig: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn raise(_sig: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    #[allow(deprecated)]
    fn setitimer(_which: c_int, _new: &itimerval, _old: Option<&mut itimerval>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn sigaction(_sig: c_int, _act: Option<&sigaction>, _oact: Option<&mut sigaction>) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn sigaltstack(_ss: Option<&stack_t>, _old_ss: Option<&mut stack_t>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn sigpending(_set: &mut sigset_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn sigprocmask(_how: c_int, _set: Option<&sigset_t>, _oset: Option<&mut sigset_t>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn sigsuspend(_mask: &sigset_t) -> Errno { Errno(ENOSYS) }
    fn sigtimedwait(_set: &sigset_t, _sig: Option<&mut siginfo_t>, _tp: Option<&timespec>) -> Result<c_int> { Err(Errno(ENOSYS)) }
}
