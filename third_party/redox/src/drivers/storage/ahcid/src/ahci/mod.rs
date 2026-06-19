use common::io::Io;
use driver_block::Disk;
use log::{error, info};

use self::disk_ata::DiskATA;
use self::disk_atapi::DiskATAPI;
use self::hba::{HbaMem, HbaPortType};

pub mod disk_ata;
pub mod disk_atapi;
pub mod fis;
pub mod hba;

pub enum AnyDisk {
    Ata(DiskATA),
    Atapi(DiskATAPI),
}
impl Disk for AnyDisk {
    fn block_size(&self) -> u32 {
        match self {
            Self::Ata(a) => a.block_size(),
            Self::Atapi(a) => a.block_size(),
        }
    }
    fn size(&self) -> u64 {
        match self {
            Self::Ata(a) => a.size(),
            Self::Atapi(a) => a.size(),
        }
    }
    async fn read(&mut self, base: u64, buffer: &mut [u8]) -> syscall::Result<usize> {
        match self {
            Self::Ata(a) => a.read(base, buffer).await,
            Self::Atapi(a) => a.read(base, buffer).await,
        }
    }
    async fn write(&mut self, base: u64, buffer: &[u8]) -> syscall::Result<usize> {
        match self {
            Self::Ata(a) => a.write(base, buffer).await,
            Self::Atapi(a) => a.write(base, buffer).await,
        }
    }
}

pub fn disks(base: usize, name: &str) -> (&'static mut HbaMem, Vec<AnyDisk>) {
    let hba_mem = unsafe { &mut *(base as *mut HbaMem) };
    hba_mem.init();
    let pi = hba_mem.pi.read();
    let disks: Vec<AnyDisk> = (0..hba_mem.ports.len())
        .filter(|&i| pi & 1 << i as i32 == 1 << i as i32)
        .filter_map(|i| {
            let port = unsafe { &mut *hba_mem.ports.as_mut_ptr().add(i) };
            let port_type = port.probe();
            info!("{}-{}: {:?}", name, i, port_type);

            let disk: Option<AnyDisk> = match port_type {
                HbaPortType::SATA => match DiskATA::new(i, port) {
                    Ok(disk) => Some(AnyDisk::Ata(disk)),
                    Err(err) => {
                        error!("{}: {}", i, err);
                        None
                    }
                },
                HbaPortType::SATAPI => match DiskATAPI::new(i, port) {
                    Ok(disk) => Some(AnyDisk::Atapi(disk)),
                    Err(err) => {
                        error!("{}: {}", i, err);
                        None
                    }
                },
                _ => None,
            };

            disk
        })
        .collect();

    (hba_mem, disks)
}
