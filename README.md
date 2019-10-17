# Du.rs
### just a new computer language project write in rust

### 1. VM
  - [ ] CPU
    - [ ] instruction
        - [x] hlt
        - [x] load
        - [x] arithmetic，，， ， float， 
            - [x] add
            - [x] sub
            - [x] mul
            - [x] div
            - [x] inc
            - [x] dec
        - [x] jump
            - [x] jmp
            - [x] jmpf
            - [x] jmpb
            - [x] jeq
            - [x] jl
            - [x] jg
        - [x] conditional
            - [x] eq
            - [x] lt
            - [x] gt
        - [x] logical
            - [x] and
            - [x] or
            - [x] xor
            - [x] not
        - [x] stack
            - [x] push
            - [x] pop
        - [x] function call
            - [x] call
            - [x] ret
        - [ ] mem
            - [ ] read
            - [ ] store
        - [ ] float
        - [ ] More...
    - [x] decode
    - [x] execute
    - [x] registers
    - [x] program
    - [x] pc
  - [ ] Mem
    - [ ] heap
        - [ ] malloc 
        - [ ] gc

### 2. REPL

  - [x] input
    - [x] .help
    - [x] .exit
    - [x] .clear
    - [ ] .load_elf
    - [ ] debug
        - [x] .history
        - [x] .program
        - [x] .registers
        - [ ] More...

### 3. Assembler 

  - [ ] lexer
    - [x] token
  - [ ] parser
    - [x] instruction
    - [x] label_declaration
    - [x] label_usage
    - [ ] directive
        - [x] .asciiz
          - [ ] Escape character
            - [ ] \n
        - [ ] .ascii
        - [x] .code
        - [x] .data
        - [ ] more
  - [ ] assembler
    - [ ] elf
        - [x] header
    - [ ] first pass
        - [ ] symbol table
            - [x] add
            - [x] get value
            - [ ] sort
            - [ ] more
    - [ ] second pass
        
