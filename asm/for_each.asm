.code
    main:   load    $0  #0
            load    $1  #50
    for:    eq      $0  $1
            prts    @hw
            dec     $1
            jne     @for
            prts    @passed
.data
    hw:     .asciiz "Hello, World."
    passed: .asciiz "Ok, 50 times print passed."
