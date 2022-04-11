    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call primary_main

# 分配初始化程序栈空间
    .section .data.boot_stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: