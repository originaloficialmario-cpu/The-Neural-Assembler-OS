ENTRY(start)

SECTIONS {
    . = 1M;

    /* Wir zwingen den gesamten .boot Abschnitt inklusive Header GANZ NACH VORNE */
    .text : {
        *(.multiboot)
        *(.text)
    }

    .data : { *(.data) }
    .bss  : { *(.bss) }
}
