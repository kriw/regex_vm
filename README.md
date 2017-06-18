# Description
VM Type Regex Engine

# BNF
```
expr    := term (| term)*
term    := factor*
factor  := Char | Char*
Char    := str  | "(" expr ")"
```
