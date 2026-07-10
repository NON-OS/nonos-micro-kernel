// NONOS std fs: files over the kernel VFS service (vfs_pool) via IPC.
// Wires open/read/write/close, seek/tell, stat, truncate, rename, unlink,
// mkdir/rmdir, and readdir to the VFS protocol. Symlinks, hard links,
// per-file times, and locks are unsupported (the VFS does not model
// them); those paths return an error or no-op.

use crate::ffi::OsString;
use crate::fmt;
use crate::fs::TryLockError;
use crate::hash::{Hash, Hasher};
use crate::io::{self, BorrowedCursor, Error, ErrorKind, IoSlice, IoSliceMut, SeekFrom};
use crate::path::{Path, PathBuf};
pub use crate::sys::fs::common::Dir;
use crate::sys::time::SystemTime;
use crate::sys::unsupported;
use crate::vec::Vec;

const MAGIC: u32 = 0x4E4F_5646;
const STATUS_OFF: usize = 20;
const BODY_OFF: usize = 24;
const OP_OPEN: u16 = 1;
const OP_CLOSE: u16 = 2;
const OP_READ: u16 = 3;
const OP_WRITE: u16 = 4;
const OP_STAT: u16 = 5;
const OP_LIST: u16 = 6;
const OP_MKDIR: u16 = 8;
const OP_UNLINK: u16 = 9;
const OP_RENAME: u16 = 10;
const OP_RMDIR: u16 = 11;
const OP_TRUNCATE: u16 = 13;
const OP_SEEK: u16 = 16;
const O_CREATE: u32 = 1;
const O_TRUNC: u32 = 1 << 1;
const O_APPEND: u32 = 1 << 2;
const SEEK_SET: u8 = 0;
const SEEK_CUR: u8 = 1;
const SEEK_END: u8 = 2;

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

unsafe fn sys3(num: i64, a1: u64, a2: u64, a3: u64) -> i64 {
    let r: i64;
    unsafe {
        core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
            in("rdx") a3, out("rcx") _, out("r11") _);
    }
    r
}

unsafe fn sys5(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> i64 {
    let r: i64;
    unsafe {
        core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
            in("rdx") a3, in("r10") a4, in("r8") a5, out("rcx") _, out("r11") _);
    }
    r
}

fn vfs_port() -> io::Result<u32> {
    let name = b"vfs_pool";
    let mut port = 0u32;
    let mut owner = 0u32;
    let rc = unsafe {
        sys5(
            tag4(b"MSVL"),
            name.as_ptr() as u64,
            name.len() as u64,
            &mut port as *mut u32 as u64,
            &mut owner as *mut u32 as u64,
            0,
        )
    };
    if rc != 0 {
        return Err(err("vfs unavailable"));
    }
    Ok(port)
}

fn getpid() -> u32 {
    let r = unsafe { sys3(tag4(b"MGPD"), 0, 0, 0) };
    if r < 0 {
        0
    } else {
        r as u32
    }
}

fn frame(op: u16, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(STATUS_OFF + body.len());
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&0u16.to_le_bytes());
    f.extend_from_slice(&0u16.to_le_bytes());
    f.extend_from_slice(&0u32.to_le_bytes());
    f.extend_from_slice(&(body.len() as u32).to_le_bytes());
    f.extend_from_slice(body);
    f
}

fn call(port: u32, op: u16, body: &[u8], reply_cap: usize) -> io::Result<Vec<u8>> {
    let tx = frame(op, body);
    let mut rx = crate::vec![0u8; BODY_OFF + reply_cap];
    let n = unsafe {
        sys5(
            tag4(b"MICL"),
            port as u64,
            tx.as_ptr() as u64,
            tx.len() as u64,
            rx.as_mut_ptr() as u64,
            rx.len() as u64,
        )
    };
    if n < (STATUS_OFF + 4) as i64 {
        return Err(err("vfs ipc failed"));
    }
    let status = i32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]);
    if status != 0 {
        return Err(vfs_err(status));
    }
    rx.truncate(n as usize);
    Ok(rx)
}

fn err(msg: &'static str) -> Error {
    Error::new(ErrorKind::Other, msg)
}

// The VFS replies with negative unix errnos; surface them as the
// ErrorKind std callers match on (fs::exists, create_new, and friends).
fn vfs_err(status: i32) -> Error {
    let kind = match status {
        -2 => ErrorKind::NotFound,
        -13 => ErrorKind::PermissionDenied,
        -17 => ErrorKind::AlreadyExists,
        -21 => ErrorKind::IsADirectory,
        -22 => ErrorKind::InvalidInput,
        -28 => ErrorKind::StorageFull,
        -39 => ErrorKind::DirectoryNotEmpty,
        _ => ErrorKind::Other,
    };
    Error::new(kind, "vfs op rejected")
}

fn path_bytes(p: &Path) -> io::Result<Vec<u8>> {
    let b = p.as_os_str().as_encoded_bytes();
    if b.is_empty() || b.len() > 255 {
        return Err(err("bad path"));
    }
    Ok(b.to_vec())
}

fn path_body(pid: u32, p: &Path) -> io::Result<Vec<u8>> {
    let path = path_bytes(p)?;
    let mut body = Vec::with_capacity(5 + path.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(&path);
    Ok(body)
}

fn read_u32(b: &[u8], off: usize) -> u32 {
    if b.len() < off + 4 {
        return 0;
    }
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

pub struct File {
    port: u32,
    pid: u32,
    fd: u32,
    // Kept for the by-path operations the protocol lacks by-fd forms of
    // (fstat, ftruncate). Renaming the file while it is open makes these
    // miss, the usual race a path-based fallback accepts.
    path: Vec<u8>,
}

#[derive(Clone)]
pub struct FileAttr {
    size: u64,
    is_dir: bool,
}

pub struct ReadDir {
    entries: crate::vec::IntoIter<DirEntry>,
}

#[derive(Clone)]
pub struct DirEntry {
    name: PathBuf,
    is_dir: bool,
}

#[derive(Clone, Debug, Default)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions {
    readonly: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType {
    is_dir: bool,
}

#[derive(Debug)]
pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn perm(&self) -> FilePermissions {
        FilePermissions { readonly: false }
    }
    pub fn file_type(&self) -> FileType {
        FileType { is_dir: self.is_dir }
    }
    pub fn modified(&self) -> io::Result<SystemTime> {
        unsupported()
    }
    pub fn accessed(&self) -> io::Result<SystemTime> {
        unsupported()
    }
    pub fn created(&self) -> io::Result<SystemTime> {
        unsupported()
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.readonly
    }
    pub fn set_readonly(&mut self, readonly: bool) {
        self.readonly = readonly;
    }
}

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
    pub fn is_file(&self) -> bool {
        !self.is_dir
    }
    pub fn is_symlink(&self) -> bool {
        false
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReadDir").finish_non_exhaustive()
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;
    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        self.entries.next().map(Ok)
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.name.clone()
    }
    pub fn file_name(&self) -> OsString {
        self.name.as_os_str().to_os_string()
    }
    pub fn metadata(&self) -> io::Result<FileAttr> {
        Ok(FileAttr { size: 0, is_dir: self.is_dir })
    }
    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(FileType { is_dir: self.is_dir })
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions::default()
    }
    pub fn read(&mut self, read: bool) {
        self.read = read;
    }
    pub fn write(&mut self, write: bool) {
        self.write = write;
    }
    pub fn append(&mut self, append: bool) {
        self.append = append;
    }
    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }
    pub fn create(&mut self, create: bool) {
        self.create = create;
    }
    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let port = vfs_port()?;
        let pid = getpid();
        // The protocol has no atomic create-exclusive; a pre-check is the
        // honest approximation and reports the common case correctly.
        if opts.create_new && stat(path).is_ok() {
            return Err(Error::new(ErrorKind::AlreadyExists, "path exists"));
        }
        let mut flags = 0u32;
        if opts.create || opts.create_new {
            flags |= O_CREATE;
        }
        if opts.truncate {
            flags |= O_TRUNC;
        }
        if opts.append {
            flags |= O_APPEND;
        }
        let pb = path_bytes(path)?;
        let mut body = Vec::with_capacity(9 + pb.len());
        body.extend_from_slice(&pid.to_le_bytes());
        body.push(pb.len() as u8);
        body.extend_from_slice(&pb);
        body.extend_from_slice(&flags.to_le_bytes());
        let rx = call(port, OP_OPEN, &body, 8)?;
        Ok(File { port, pid, fd: read_u32(&rx, BODY_OFF), path: pb })
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        stat(Path::new(crate::str::from_utf8(&self.path).map_err(|_| err("bad path"))?))
    }

    fn seek_raw(&self, whence: u8, offset: i64) -> io::Result<u64> {
        let mut body = Vec::with_capacity(21);
        body.extend_from_slice(&self.pid.to_le_bytes());
        body.extend_from_slice(&self.fd.to_le_bytes());
        body.push(whence);
        body.extend_from_slice(&offset.to_le_bytes());
        let rx = call(self.port, OP_SEEK, &body, 8)?;
        let b = &rx[BODY_OFF..];
        if b.len() < 8 {
            return Err(err("short seek reply"));
        }
        Ok(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let want = buf.len().min(65536) as u32;
        let mut body = Vec::with_capacity(12);
        body.extend_from_slice(&self.pid.to_le_bytes());
        body.extend_from_slice(&self.fd.to_le_bytes());
        body.extend_from_slice(&want.to_le_bytes());
        let rx = call(self.port, OP_READ, &body, 65536)?;
        let data = &rx[BODY_OFF..];
        let n = data.len().min(buf.len());
        buf[..n].copy_from_slice(&data[..n]);
        Ok(n)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let mut body = Vec::with_capacity(8 + buf.len());
        body.extend_from_slice(&self.pid.to_le_bytes());
        body.extend_from_slice(&self.fd.to_le_bytes());
        body.extend_from_slice(buf);
        let rx = call(self.port, OP_WRITE, &body, 8)?;
        Ok(read_u32(&rx, BODY_OFF) as usize)
    }

    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_>) -> io::Result<()> {
        let mut tmp = [0u8; 4096];
        let n = self.read(&mut tmp)?;
        cursor.append(&tmp[..n]);
        Ok(())
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        for b in bufs {
            if !b.is_empty() {
                return self.read(b);
            }
        }
        Ok(0)
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        for b in bufs {
            if !b.is_empty() {
                return self.write(b);
            }
        }
        Ok(0)
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn fsync(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn datasync(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn lock(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn lock_shared(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn try_lock(&self) -> Result<(), TryLockError> {
        Ok(())
    }

    pub fn try_lock_shared(&self) -> Result<(), TryLockError> {
        Ok(())
    }

    pub fn unlock(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn truncate(&self, size: u64) -> io::Result<()> {
        let mut body = Vec::with_capacity(13 + self.path.len());
        body.extend_from_slice(&self.pid.to_le_bytes());
        body.push(self.path.len() as u8);
        body.extend_from_slice(&self.path);
        body.extend_from_slice(&size.to_le_bytes());
        call(self.port, OP_TRUNCATE, &body, 0).map(|_| ())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let (whence, offset) = match pos {
            SeekFrom::Start(n) => {
                let n = i64::try_from(n).map_err(|_| err("seek offset too large"))?;
                (SEEK_SET, n)
            }
            SeekFrom::Current(n) => (SEEK_CUR, n),
            SeekFrom::End(n) => (SEEK_END, n),
        };
        self.seek_raw(whence, offset)
    }

    pub fn size(&self) -> Option<io::Result<u64>> {
        Some(self.file_attr().map(|a| a.size()))
    }

    pub fn tell(&self) -> io::Result<u64> {
        self.seek_raw(SEEK_CUR, 0)
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unsupported()
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        Ok(())
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let mut body = [0u8; 8];
        body[..4].copy_from_slice(&self.pid.to_le_bytes());
        body[4..8].copy_from_slice(&self.fd.to_le_bytes());
        let _ = call(self.port, OP_CLOSE, &body, 0);
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }
    pub fn mkdir(&self, p: &Path) -> io::Result<()> {
        let port = vfs_port()?;
        call(port, OP_MKDIR, &path_body(getpid(), p)?, 0).map(|_| ())
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File").field("fd", &self.fd).finish()
    }
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    let port = vfs_port()?;
    let rx = call(port, OP_LIST, &path_body(getpid(), p)?, 65536)?;
    let mut out: Vec<DirEntry> = Vec::new();
    let mut body = &rx[BODY_OFF..];
    while !body.is_empty() {
        let len = body[0] as usize;
        if body.len() < 1 + len {
            break;
        }
        if let Ok(name) = crate::str::from_utf8(&body[1..1 + len]) {
            let is_dir = name.ends_with('/');
            let trimmed = name.trim_end_matches('/');
            out.push(DirEntry { name: PathBuf::from(trimmed), is_dir });
        }
        body = &body[1 + len..];
    }
    Ok(ReadDir { entries: out.into_iter() })
}

pub fn unlink(p: &Path) -> io::Result<()> {
    let port = vfs_port()?;
    call(port, OP_UNLINK, &path_body(getpid(), p)?, 0).map(|_| ())
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    let port = vfs_port()?;
    let rx = call(port, OP_STAT, &path_body(getpid(), p)?, 16)?;
    let b = &rx[BODY_OFF..];
    if b.len() < 12 {
        return Err(err("short stat"));
    }
    let size = u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
    let flags = read_u32(b, 8);
    Ok(FileAttr { size, is_dir: flags & 1 != 0 })
}

pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    stat(p)
}

pub fn exists(p: &Path) -> io::Result<bool> {
    Ok(stat(p).is_ok())
}

pub fn rmdir(p: &Path) -> io::Result<()> {
    let port = vfs_port()?;
    call(port, OP_RMDIR, &path_body(getpid(), p)?, 0).map(|_| ())
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    let port = vfs_port()?;
    let ob = path_bytes(old)?;
    let nb = path_bytes(new)?;
    let mut body = Vec::with_capacity(6 + ob.len() + nb.len());
    body.extend_from_slice(&getpid().to_le_bytes());
    body.push(ob.len() as u8);
    body.extend_from_slice(&ob);
    body.push(nb.len() as u8);
    body.extend_from_slice(&nb);
    call(port, OP_RENAME, &body, 0).map(|_| ())
}

pub fn set_perm(_p: &Path, _perm: FilePermissions) -> io::Result<()> {
    Ok(())
}

pub fn set_times(_p: &Path, _times: FileTimes) -> io::Result<()> {
    Ok(())
}

pub fn set_times_nofollow(_p: &Path, _times: FileTimes) -> io::Result<()> {
    Ok(())
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    let mut ro = OpenOptions::new();
    ro.read(true);
    let mut data = Vec::new();
    {
        let f = File::open(from, &ro)?;
        let mut chunk = [0u8; 4096];
        loop {
            let n = f.read(&mut chunk)?;
            if n == 0 {
                break;
            }
            data.extend_from_slice(&chunk[..n]);
        }
    }
    let mut wo = OpenOptions::new();
    wo.write(true);
    wo.create(true);
    wo.truncate(true);
    let out = File::open(to, &wo)?;
    out.write(&data)?;
    Ok(data.len() as u64)
}
