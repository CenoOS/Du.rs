# DuVM
### just a new computer language project write in rust
![DuVM](./assets/logo.svg)
### 1. VM
  - [ ] CPU
    - [ ] instruction
        - [x] hlt
        - [x] load
        - [x] arithmetic
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
    - [x] sp
    - [x] bp
    - [x] stack
  - [ ] Mem
    - [ ] ro
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
        
### 4. Dulang
- [x] EBNF
```
Tokens:

Grouping tokens:

( ) [ ] { }

Unary/binary operators:

+ - ! ~ & *

LSHIFT = '<<'
RSHIFT = '>>'
EQ = '=='
NOTEQ = '!='
LTEQ = '<='
RTEQ = '>='
AND = '&&'
OR = '||'

+ - | ^ LSHIFT RSHIFT
* / % &
EQ NOTEQ < LTEQ > RTEQ
AND
OR
? :

Assignment operators:

COLON_ASSIGN  = ':='
ADD_ASSIGN  = '+='
SUB_ASSIGN  = '-='
AND_ASSIGN = '&='
OR_ASSIGN  = '|='
XOR_ASSIGN  = '^='
LSHIFT_ASSIGN  = '<<='
RSHIFT_ASSIGN  = '>>='
MUL_ASSIGN  = '*='
DIV_ASSIGN  = '/='
MOD_ASSIGN  = '%='
=

INC = '++'
DEC = '--'

Names/literals:

NAME = [a-zA-Z_][a-zA-Z0-9_]*
INT = [1-9][0-9]* | 0[xX][0-9a-fA-F]+ | 0[0-7]+ | 0[0bB][0-1]+
FLOAT = [0-9]*[.][0-9]*([eE][+-]?[0-9]+)?
CHAR = '\'' . '\''
STR = '"' [^"]* '"'



EBNF grammer:

type_list = type (',' type)*
name_list = NAME (',' NAME)*

base_type = NAME
			| 'func' '(' type_list? ')' (':' type)?
			| '(' base_type ')'

type = base_type ('[' expr ']' | '*')*

enum_item = NAME ('=' expr)?
enum_items = enum_item (',' enum_item)* ','?
enum_decl = NAME '{' enum_items? '}'


aggregate_field = name_list ':' type
aggregate_decl = NAME '{' (aggregate_field ';')* '}'

var_decl = NAME '=' expr 
			| NAME ':' type ('=' expr)?

const_decl = NAME '=' expr

typedef_decl = NAME '=' type

func_param = NAME ':' type
func_param_list = func_param (',' func_param)*
func_decl = NAME '(' func_param_list?  ')' (':' type)? '{' stmt_block '}'


decl	='enum' enum_decl
		|'struct' aggregate_decl
		|'union' aggregate_decl
		|'var' var_decl
		|'const' const_decl
		|'typedef' typedef_decl
		|'func' func_decl

Statements:

assign_op = '=' | COLON_ASSIGN | ADD_ASSIGN |....

switch_case = (CASE expr | DEFAULT) ':' stmt*
switch_block = '{' switch_case* '}'

stmt = 'return' expr
		| 'if' '(' expr ')' stmt_block ('else' 'if' '(' expr ')' stmt_block)* ('else' stmt_block)?
		| 'while' '(' expr ')' stmt_block
		| 'for' '(' stmt_list ';' expr ';' stmt_list ')' stmt_block
		| 'do' stmt_block 'while' '(' expr ')' ';'
		| 'break' ';'
		| 'continue' ';'
		| '{' stmt* '}'
		| 'switch' '(' expr ')' switch_block
		| expr (INC | DEC | assign_op expr)?

type_spec = NAME | '(' ':' type ')'

operand_expr = NAME
				|  INT
				| FLOAT 
				| STR
				| CAST '(' type ')' expr
				| '(' expr ')'
				| type_spec? '{' expr_list '}'




base_expr = operand_expr ('(' expr_list ')' | '[' expr ']' | '.' NAME)*

unary_expr = [+-&*] unary_expr
			| base_expr

mul_op = '*' | '/' | '%' | '&' | LSHIFT | RSHIFT
mul_expr = unary_expr(mul_op unary_expr)*
add_op = '+' | '-' | '|' | '^'
add_expr = mul_expr (add_op mul_expr)*
cmp_op = EQ | NOTEQ | LTEQ | GTEQ | '<' | '>'
cmp_expr = add_expr (cmp_op add_expr)*
and_expr = cmp_expr (AND cmp_expr)*
or_expr = and_expr (OR and_expr)*
ternary_expr = or_expr ('?' ternary_expr ':' ternary_expr)?
expr = ternary_expr
```
- [x] Lexer
    - [x] tokens
        - [x] Grouping tokens
            - [x] ( ) [ ] { }
        - [x] Unary/binary operators
            - [x] \+ - ! ~ & *
            - [x] LSHIFT = '<<'
            - [x] RSHIFT = '>>'
            - [x] EQ = '=='
            - [x] NOTEQ = '!='
            - [x] LTEQ = '<='
            - [x] RTEQ = '>='
            - [x] AND = '&&'
            - [x] OR = '||'
            - [x] \+ - | ^ LSHIFT RSHIFT
            - [x] \* / % &
            - [x] EQ NOTEQ < LTEQ > RTEQ
            - [x] AND
            - [x] OR
            - [x] ? :
        - [x] Assignment operators
            - [x] COLON_ASSIGN  = ':='
            - [x] ADD_ASSIGN  = '+='
            - [x] SUB_ASSIGN  = '-='
            - [x] AND_ASSIGN = '&='
            - [x] OR_ASSIGN  = '|='
            - [x] XOR_ASSIGN  = '^='
            - [x] LSHIFT_ASSIGN  = '<<='
            - [x] RSHIFT_ASSIGN  = '>>='
            - [x] MUL_ASSIGN  = '*='
            - [x] DIV_ASSIGN  = '/='
            - [x] MOD_ASSIGN  = '%='
            - [x] =
            - [x] INC = '++'
            - [x] DEC = '--'
        - [x] Names/literals
            - [x] NAME = [a-zA-Z_][a-zA-Z0-9_]*
            - [x] INT = [1-9][0-9]* | 0[xX][0-9a-fA-F]+ | 0[0-7]+ | 0[0bB][0-1]+
            - [x] FLOAT = [0-9]*[.][0-9]*([eE][+-]?[0-9]+)?
            - [x] CHAR = '\'' . '\''
            - [x] STR = '"' [^"]* '"
- [ ] Parser
    - [ ] grammer
        - [ ] type_list      
        - [ ] name_list
        - [ ] base_type
        - [ ] type
        - [ ] enum_item
        - [ ] enum_items
        - [ ] enum_decl
        - [ ] aggregate_field
        - [ ] aggregate_decl
        - [ ] var_decl
        - [ ] const_decl
        - [ ] typedef_decl
        - [ ] func_param
        - [ ] func_param_list
        - [ ] func_decl
        - [ ] decl
    - [ ] Statements
        - [ ] assign_op
        - [ ] switch_case
        - [ ] switch_block
        - [ ] stmt
        - [ ] type_spec
        - [ ] operand_expr
        - [ ] base_expr
        - [ ] unary_expr
        - [ ] mul_op
        - [ ] mul_expr   
        - [ ] add_op
        - [ ] add_expr
        - [ ] cmp_op
        - [ ] cmp_expr
        - [ ] and_expr
        - [ ] or_expr
        - [ ] ternary_expr
        - [ ] expr 
- [ ] Interpreter
- [ ] Compiler
- [ ] JIT
