const BGA_INDEX_XRES: u16 = 1;
const BGA_INDEX_YRES: u16 = 2;
const BGA_INDEX_BPP: u16 = 3;
const BGA_INDEX_ENABLE: u16 = 4;

pub struct Bga {
    bar: *mut u8,
}

impl Bga {
    pub unsafe fn new(bar: *mut u8) -> Bga {
        Bga { bar }
    }

    fn bochs_dispi_addr(&mut self, index: u16) -> *mut u16 {
        assert!(index <= 0x10);
        unsafe {
            self.bar
                .byte_add(0x500)
                .cast::<u16>()
                .add(usize::from(index))
        }
    }

    fn bochs_dispi_read(&mut self, index: u16) -> u16 {
        unsafe { self.bochs_dispi_addr(index).read_volatile() }
    }

    fn bochs_dispi_write(&mut self, index: u16, data: u16) {
        assert!(index <= 0x10);
        unsafe {
            self.bochs_dispi_addr(index).write_volatile(data);
        }
    }

    pub fn width(&mut self) -> u16 {
        self.bochs_dispi_read(BGA_INDEX_XRES)
    }

    pub fn height(&mut self) -> u16 {
        self.bochs_dispi_read(BGA_INDEX_YRES)
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.bochs_dispi_write(BGA_INDEX_ENABLE, 0);
        self.bochs_dispi_write(BGA_INDEX_XRES, width);
        self.bochs_dispi_write(BGA_INDEX_YRES, height);
        self.bochs_dispi_write(BGA_INDEX_BPP, 32);
        self.bochs_dispi_write(BGA_INDEX_ENABLE, 0x41);
    }
}
