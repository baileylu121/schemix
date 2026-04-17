            Context Free Grammar

Start
1*2+3*4+5
symbol

                  <exp> ::= <factor>

Production
| <exp> + <factor>
Rule
<factor> ::= NUMBER
Nonterminal
| <factor> \* NUMBER

         metasymbol             Terminal

Parse Tree

ADD_OP: "+" | "-"
MUL_OP: "\*" | "/"

add_expr: add_expr ADD_OP mul_expr
| mul_expr

mul_expr: mul_expr MUL_OP NUMBER
| NUMBER

expr: add_expr
