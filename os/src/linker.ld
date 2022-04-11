OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    s_kernel = .;

    /* 代码段 */
    s_text = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    e_text = .;

    /* 已初始化的全局数据 - 只读 */
    s_rodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);
    e_rodata = .;

    /* 已初始化的全局数据 - 可修改 */
    s_data = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    e_data = .;

    /* 未初始化数据段 */
    s_bss = .;
    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    . = ALIGN(4K);
    e_bss = .;

    e_kernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
