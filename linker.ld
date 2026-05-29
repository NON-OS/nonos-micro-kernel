ENTRY(_start)
OUTPUT_ARCH(i386:x86-64)
OUTPUT_FORMAT(elf64-x86-64)

/* Static kernel image, fixed at the canonical upper-half kernel
 * window 0xFFFFFFFF80000000+. Linked with -no-pie, code-model=kernel,
 * no .dynamic, no .rela.*, no .got. The bootloader allocates phys
 * frames, copies each PT_LOAD verbatim, and maps phys -> p_vaddr
 * with the correct permissions before jumping. */

PHDRS {
    text   PT_LOAD FLAGS(5);  /* r-x */
    rodata PT_LOAD FLAGS(4);  /* r-- */
    data   PT_LOAD FLAGS(6);  /* rw- */
}

SECTIONS {
    . = 0xFFFFFFFF80000000;
    __kernel_image_start = .;

    .multiboot : {
        KEEP(*(.multiboot))
    } :text

    .text ALIGN(0x1000) : {
        *(.text._start)
        *(.text .text.*)
        *(.ltext .ltext.*)
    } :text

    .rodata ALIGN(0x1000) : {
        *(.rodata .rodata.*)
        *(.lrodata .lrodata.*)
    } :rodata

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

    /* Layout-stable observation ring for the `dbg-ring` feature.
     * Pinned ahead of .bss with its own ALIGN so that growing other
     * statics does not shift the ring's VA. Tagged NOBITS via
     * NOLOAD-equivalent: the ring is zero-initialised at the call
     * site (magic field flips from 0 to "NDBG" on first emit). */
    .nonos.dbg_ring ALIGN(0x1000) : {
        __nonos_dbg_ring_start = .;
        KEEP(*(.nonos.dbg_ring))
        __nonos_dbg_ring_end = .;
    } :data

    /* BSS is the LAST loadable section in :data so it stays NOBITS
     * and the kernel image stays small no matter how much zero-init
     * static data the kernel grows. */
    .bss ALIGN(0x1000) : {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    } :data

    . = ALIGN(0x1000);
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
    }
}
