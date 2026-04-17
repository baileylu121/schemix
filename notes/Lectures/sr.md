a S0 0
:=
b
^
c
\n

1 -shift

     a     S0
     :=    S1 a   01
     b
     ^
     c
     \n

a
:= S0
01
b S1 a
^
c
\n

     2 - shift

a
:= 014
S0
b S1 a
^ S4 :=
c
\n
 3 - shift
a
:= S0 0147
b S1 a
^ S4 :=
c S7 b
\n
If the action table entry is reduce r and rule r has m symbols in its RHS, then
pop m symbols off the parse stack. Let s' be the state now revealed on top of the
parse stack and N be the LHS nonterminal for rule r.
Then consult the goto table and push the state given by goto[s'][N] onto the stack.
The lookahead token is not changed by this step.
 4 - reduce R6 ( b is a unary)

a
S0 0 1 4 7 10
:=
b S1 a
^ S4 :=
c S10 b unary
\n
 5 - shift

                 S0
                 S1 a
                 S4 :=
                 S10 b unary
                 S12 ^

a
:=
0 1 4 7 10 12
b
^
c
\n
 6 - shift

           S0
           S1 a
           S4 :=
           S10 b unary
           S12 ^
           S7 c

a
:=
0 1 4 7 10 12 7
b
^
c
\n
 7 – reduce c is a unary

     S0
     S1 a
     S4 :=
     S10 b unary
     S12 ^
     S10 c unary

a
:=
0 1 4 7 10 12 7 10
b
^
c
\n
 8 - reduce R4 (unary is an expr)

                 S0
                 S1 a
                 S4 :=
                 S10 b unary
                 S12 ^
                 S13 c expr

a
:= 0 1 4 7 10 12 7 10 13
b
^
c
\n
 9 - reduce R3 ( b ^ c is an expr)

        S0
        S1 a
        S4 :=
        S9 expr

a
:=
b 0 1 4 7 10 12 7 10 13 9
^
c
\n
 10 - reduce R2 (a:=b^c is a stmt)

     S0
     S3 stmt

a
:=
b 0 1 4 7 10 12 7 10 13 9 3 ……..
^
c
\n
