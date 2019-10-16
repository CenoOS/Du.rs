.code
    main:   load    $0  #0
            load    $1  #50
            load    $2  #0
    for:    eq      $0  $1
            prts    @hw
            dec     $1
            inc     $2
            jne     @for
            prts    @passed
            hlt
.data
    hw:     .asciiz "Hello, World.\n"
    passed: .asciiz "Ok, 50 times print passed."
