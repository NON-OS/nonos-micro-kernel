ENTRY(_start)
OUTPUT_ARCH(aarch64)
OUTPUT_FORMAT(elf64-littleaarch64)

/* aarch64 kernel image. Linked at the standard QEMU virt board RAM +
 * 512 KiB to leave room for the bootloader-supplied DTB at the bottom
 * of RAM. On real hardware the loader places PT_LOAD segments at
 * matching physical addresses per board profile; the kernel runs
 * identity-mapped at this VA until mmu::init_mmu installs the
 * high-half mapping. */

PHDRS {
    text   PT_LOAD FLAGS(5);  /* r-x */
    rodata PT_LOAD FLAGS(4);  /* r-- */
    data   PT_LOAD FLAGS(6);  /* rw- */
}

SECTIONS {
    . = 0x40080000;
    __kernel_image_start = .;

    .text ALIGN(0x1000) : {
        *(.text._start)
        *(.text .text.*)
        *(.ltext .ltext.*)
    } :text

    .rodata ALIGN(0x1000) : {
        *(.rodata .rodata.*)
        *(.lrodata .lrodata.*)
    } :rodata

    /* Writable data starts on a 2MB boundary so the boot identity map, which
       uses 2MB blocks, can keep text read only and executable without also
       making .data and .bss read only. */
    . = ALIGN(0x200000);
    __kernel_rw_start = .;

    .data ALIGN(0x1000) : {
        *(.data .data.*)
    } :data

    .nonos.manifest ALIGN(0x1000) : {
        __nonos_manifest_start = .;
        KEEP(*(.nonos.manifest))
        __nonos_manifest_end = .;
    } :data

    .nonos.sig ALIGN(8) : {
        __nonos_signature_start = .;
        KEEP(*(.nonos.sig))
        __nonos_signature_end = .;
    } :data

    .bss ALIGN(0x1000) : {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(8);
        __bss_end = .;
    } :data

    /* 2MB so the boot identity map, which uses 2MB blocks, can mark the text
       below executable without making the stack executable with it. */
    . = ALIGN(0x200000);
    __stack_bottom = .;
    . += 64K;
    __stack_top = .;

    __kernel_image_end = .;
    _end = .;

    /DISCARD/ : {
        *(.eh_frame*)
        *(.comment)
        *(.note*)
        *(.debug*)
        *(.plt*)
        *(.dynamic)
        *(.dynsym)
        *(.dynstr)
        *(.hash)
        *(.gnu.hash)
        *(.rela.*)
        *(.got)
        *(.got.plt)
        *(.interp)
    }
}
