// +
        movq    -8(%rbp), %rax
        movzbl  (%rax), %eax
        addl    ${TIMES}, %eax
        movl    %eax, %edx
        movq    -8(%rbp), %rax
        movb    %dl, (%rax)
