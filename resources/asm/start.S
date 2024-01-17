    .globl main
get_character:
        pushq   %rbp
        movq    %rsp, %rbp
        subq    $16, %rsp
        movl    $0, %eax
        call    getchar
        movl    %eax, -8(%rbp)
        movl    -8(%rbp), %eax
        movl    %eax, -4(%rbp)
        jmp     .CLEAR_BUFFER_CONDITION

.CLEAR_BUFFER:
        movl    $0, %eax
        call    getchar
        movl    %eax, -4(%rbp)

.CLEAR_BUFFER_CONDITION:
        cmpl    $10, -4(%rbp)
        jne     .CLEAR_BUFFER
        movl    -8(%rbp), %eax
        leave
        ret

main:
        pushq   %rbp
        movq    %rsp %rbp
        subq    $30016, %rsp
        movq    $0, -30016(%rbp)
        movq    $0, -30008(%rbp)
        leaq    -30000(%rbp), %rax
        movl    $29984, %edx
        movl    $0, %esi
        movq    %rax, %rdi
        call    memset
        leaq    -30016(%rbp), %rax
        movq    %rax, -8(%rbp)
