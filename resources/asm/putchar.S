// .
        movq    -8(%rbp), %rax
        movzbl  (%rax), %eax
        movsbl  %al, %eax
        movl    %eax, %edi
        call    putchar
