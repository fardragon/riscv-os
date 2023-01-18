ENTRY(_start);

SECTIONS {
    . = 0x80200000;
    .text : {
        . = ALIGN(0x1000);
        PROVIDE(__text_start = .);
        *(.text.init)
        *(.text .text.*)
        PROVIDE(__text_end = .);
    }

    .data : {
        . = ALIGN(0x1000);
        PROVIDE(__data_start = .);
        *(.data .data.* .rodata .rodata.*)
        . = ALIGN(8);
        PROVIDE(__global_pointer$ = .);
        *(.sdata .sdata.*)
        . = ALIGN(4);
        PROVIDE(__bss_start = .);
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        . = ALIGN(4);
        PROVIDE(__bss_end = .);
        PROVIDE(__data_end = .);
    }

    .kernel_stack : ALIGN(8) {
        PROVIDE(__stack_end = .);
        . += 4096 * 4;
        PROVIDE(__stack_start = .);
    }

    PROVIDE(__kernel_end = .);

    /DISCARD/ : { *(.eh_frame_hdr .eh_frame) }
}