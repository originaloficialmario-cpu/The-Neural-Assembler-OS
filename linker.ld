/* linker.ld - Der Speichermap-Erzwinger */
ENTRY(start)

SECTIONS {
    /* Wir zwingen das OS, exakt bei 1 Megabyte im RAM zu starten */
    . = 1M;

    /* Der Türsteher (boot.asm) MUSS als Erstes kommen! */
    .boot : {
        *(.multiboot)
    }

    /* Danach kommt unser Rust-Kernel-Code */
    .text : {
        *(.text)
    }

    /* Zuletzt der reservierte Speicherplatz */
    .bss : {
        *(.bss)
    }
}
