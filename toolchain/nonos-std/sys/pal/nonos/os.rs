// NONOS pal os: real process exit (mk_exit) and pid (mk_getpid). Every
// capsule sees the VFS from its root, so the working directory is the
// fixed "/" and temp_dir is the store's /tmp; chdir and the executable
// path stay unsupported (the capsule model has neither).

use super::unsupported;
use crate::ffi::{OsStr, OsString};
use crate::marker::PhantomData;
use crate::path::{self, PathBuf};
use crate::{fmt, io};

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_EXIT: i64 = tag4(b"MEXT");
const N_MK_GETPID: i64 = tag4(b"MGPD");

pub fn getcwd() -> io::Result<PathBuf> {
    // NONOS capsules run with a fixed root working directory. Returning "/"
    // rather than an error lets unmodified crates.io tools that query the CWD
    // at startup (ripgrep among them) initialize instead of bailing out with
    // "failed to get current working directory".
    Ok(PathBuf::from("/"))
}

pub fn chdir(_: &path::Path) -> io::Result<()> {
    unsupported()
}

pub struct SplitPaths<'a>(!, PhantomData<&'a ()>);

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
    panic!("unsupported")
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.0
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on this platform yet".fmt(f)
    }
}

impl crate::error::Error for JoinPathsError {}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

pub fn temp_dir() -> PathBuf {
    PathBuf::from("/tmp")
}

pub fn home_dir() -> Option<PathBuf> {
    None
}

pub fn exit(code: i32) -> ! {
    unsafe {
        core::arch::asm!("syscall", in("rax") N_MK_EXIT, in("rdi") code as u64, options(noreturn));
    }
}

pub fn getpid() -> u32 {
    let r: i64;
    unsafe {
        core::arch::asm!("syscall", inout("rax") N_MK_GETPID => r, out("rcx") _, out("r11") _);
    }
    if r < 0 { 0 } else { r as u32 }
}
