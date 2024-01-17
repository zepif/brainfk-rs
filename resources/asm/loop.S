.{LABEL_COND}:
        movq    -8(%rbp), %rax
        movzbl  (%rax), %eax
        testb   %al, %al
        jne     .{LABEL_BODY}
