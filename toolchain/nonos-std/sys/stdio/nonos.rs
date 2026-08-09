// NONOS std PAL: stdout/stderr via the kernel stdout syscall, which mirrors
// the bytes into this process's `proc.<pid>` inbox for its launcher to drain.
// Stdin is a blocking read of this process's kernel stdin channel, fed by a
// launcher (the terminal).
// Raw syscall: rax = tag, rdi/rsi = (buf, len).

use crate::io;

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_STDOUT_WRITE: i64 = tag4(b"MSOW");
const N_MK_STDIN_READ: i64 = tag4(b"MSRD");
const N_MK_YIELD: i64 = tag4(b"MYLD");
const CHUNK: usize = 240;

// Drain one chunk of this process's stdin channel. The kernel read is
// non-blocking: it returns the bytes a launcher (the terminal) has fed to
// `stdin.<pid>`, 0 when nothing is queued yet, or a negative errno.
#[inline]
unsafe fn stdin_read(ptr: *mut u8, len: usize) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") N_MK_STDIN_READ => ret,
            in("rdi") ptr as u64,
            in("rsi") len as u64,
            out("rcx") _,
            out("r11") _,
        );
    }
    ret
}

#[inline]
fn raw_yield() {
    unsafe {
        core::arch::asm!("syscall", in("rax") N_MK_YIELD, out("rcx") _, out("r11") _);
    }
}

// Program stdout. Unlike the kernel debug syscall this needs no `Capability::Debug`:
// the kernel only mirrors the bytes into this process's `proc.<pid>` inbox,
// which its launcher drains.
#[inline]
unsafe fn stdout_write(ptr: *const u8, len: usize) {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") N_MK_STDOUT_WRITE => ret,
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
    // Blocking read, the std contract. The kernel channel is non-blocking, so
    // spin with a yield until a launcher feeds a chunk. A negative return is
    // an unreadable channel, reported as end of input rather than an error so
    // a plain `read_to_string` terminates instead of looping.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            let n = unsafe { stdin_read(buf.as_mut_ptr(), buf.len()) };
            if n > 0 {
                return Ok(n as usize);
            }
            if n < 0 {
                return Ok(0);
            }
            raw_yield();
        }
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
            unsafe { stdout_write(buf[off..].as_ptr(), n) };
            off += n;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 8 * 1024;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<Vec<u8>> {
    Some(Vec::new())
}
