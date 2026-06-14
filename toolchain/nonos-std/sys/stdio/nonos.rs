// NONOS std PAL: stdout/stderr via the kernel debug syscall (serial sink),
// the same path the userland libc `mk_debug` uses. Stdin is empty until a
// console source is wired. Raw syscall: rax = tag, rdi/rsi = (buf, len).

use crate::io;

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_DEBUG: i64 = tag4(b"MDBG");
const CHUNK: usize = 240;

#[inline]
unsafe fn debug_write(ptr: *const u8, len: usize) {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") N_MK_DEBUG => ret,
            in("rdi") ptr as u64,
            in("rsi") len as u64,
            out("rcx") _,
            out("r11") _,
        );
    }
    let _ = ret;
}

pub struct Stdin;
pub struct Stdout;
pub type Stderr = Stdout;

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut off = 0;
        while off < buf.len() {
            let n = core::cmp::min(CHUNK, buf.len() - off);
            unsafe { debug_write(buf[off..].as_ptr(), n) };
            off += n;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<Vec<u8>> {
    Some(Vec::new())
}
