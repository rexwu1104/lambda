Statement
    ::= Define
    ::= Decorators Define

Decorators ::= "@" Identifier

Define
    ::= DefineMacro
    ::= DefineFunction
    ::= DefineType
    ::= DefineVariable

DefineMacro
    ::= "`" Identifier "`" "of" Params Any
    ::= "`" Identifier "`" "of" Params Anys "end"

DefineFunction
    ::= Identifier "of" Params Expression
    ::= Identifier "of" Params Expressions "end"
    ::= Identifier "of" Params Conditions "end"

Conditions
    ::= Conditions "," Condition
    ::= Condition

Condition ::= Cond "=>" Expressions
Cond
    ::= Value
    ::= Value CmpOp
    ::= CmpOp Value
    ::= Value CmpOp "$" CmpOp Value
    ::= Cond CondConcat Cond
    ::= Range
    ::= "?"
    ::= "_"

CondConcat
    ::= "|"
    ::= "&"

Range
    ::= "(" Value "," Value ")"
    ::= "[" Value "," Value ")"
    ::= "(" Value "," Value "]"
    ::= "[" Value "," Value "]"

DefineType ::= "type" Identifier TypeStruct
TypeStruct
    ::= "(" Types ")"
    ::= "(" TypePairs ")"
    ::= "=" ClassBody "end"

Types
    ::= Types Type
    ::= Type

TypePair ::= Identifier ":" Type
TypePairs
    ::= TypePairs TypePair
    ::= TypePair

ClassBody ::= Bodys
Bodys
    ::= Bodys Body
    ::= Body

Body
    ::= Identifier
    ::= Statement

DefineVariable ::= Identifier "=" Value

Expressions
    ::= Expressions ";" Expression
    ::= Expression

Expression
    ::= CallFunction
    ::= DefineVariable
    ::= DefineFunction

CallFunction
    ::= Identifier "(" ")"
    ::= Identifier Values

Values
    ::= Values Value
    ::= Value

Value
    ::= Identifier // builtin
    ::= Number // builtin
    ::= Float // builtin
    ::= Char // builtin
    ::= String // builtin
    ::= Boolean // builtin
    ::= Tuple
    ::= Array
    ::= Lambda
    ::= Value Symbol Value
    ::= Symbol Value

Tuple ::= "(" Values ")"
Array ::= "[" Values "]"
Lambda ::= "(" Params ")" "->" Expressions "end"

Param ::= Identifier
Params
    ::= Params Param
    ::= Param
