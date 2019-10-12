# Du.rs
### just a new computer language project write in rust

### 1. VM
  - [ ] CPU
    - [ ] instruction
        - [x] hlt
        - [x] load
        - [x] add
        - [x] sub
        - [x] mul
        - [x] div
        - [x] jmp
        - [x] jmpf
        - [x] jmpb
        - [x] eq
        - [x] jeq
        - [x] inc
        - [x] dec
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
        
