ENTRY(start)

SECTIONS {
    . = 0x100000; /* 1 MB */

    .text : ALIGN(4K) {
        KEEP(*(.multiboot))
        *(.text)
    }

    /* Texte strikt vom Code separieren */
    .rodata : ALIGN(4K) {
        *(.rodata*)
    }

    .data : ALIGN(4K) {
        *(.data)
    }

    .bss : ALIGN(4K) {
        *(.bss)
        *(COMMON)
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.note.gnu.property)
        *(.comment)
    }
}
