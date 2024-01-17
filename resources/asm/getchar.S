// ,
        movl    $0, %eax
        call    get_character
        movq    -8(%rbp), %rdx
        movb    %al, (%rdx)
