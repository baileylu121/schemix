                      Backus-Naur Form

Backus-Naur Form (BNF), created around 1960 to describe Algol 60
and named after two members of the Algol 60 committee

In order to write a parser, we need some way to describe the rules,
the grammar, the parser uses to turn a sequence of tokens into a parse tree.

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

Associativity and Precedence

Operator precedence. Operator precedence
specifies the manner in which operands are
grouped with operators. For example, 1 + 2 _ 3
is treated as 1 + (2 _ 3), whereas 1 _ 2 + 3 is
treated as (1 _ 2) + 3 because the multiplication
operator has a higher precedence than the
addition operator. You can use parentheses to
override the default operator precedence rules.
Operator associativity. When an expression has
two operators with the same precedence, the
operators and operands are grouped according
to their associativity. For example 72 / 2 /3 is treated as (72 / 2) / 3
since the division
operator is left-to-right associate. You can
use parentheses to override the default operator
associativity rules.

                            Operators may be associative (meaning the
                            operations can be grouped arbitrarily),
                            left-associative (meaning the operations are
                            grouped from the left), right-associative
                            (meaning the operations are grouped from the
                            right) or non-associative (meaning operations
                            cannot be chained, often because the output type
                            is incompatible with the input types).

Associativity and Precedence examples

(1-2)-3 = -4 In order to reflect normal usage, addition,
1-(2-3) = 2 subtraction, multiplication, and division
operators are usually left-associative
2*(3+4) = 14
(2*3)+4 = 10

(2^3)^4 = 4096
2^(3^4) = 2417851639229258349412352

                                        for an exponentiation operator there
                                        is no general agreement

Associativity and Precedence

Due to roundoff errors, the associative laws of algebra do not necessarily hold
for floating-point numbers. For example, the expression (x+y)+z has a totally
different answer than x+(y+z) when x = 1e30, y = -1e30 and z = 1 (it is 1 in the
former case, 0 in the latter).
In the formal language theory of computer science, left recursion is a special
case of recursion where a string is recognized as part of a language by the fact
that it
decomposes into a string from that same
language (on the left) and a suffix (on the right).
For instance, 1 + 2 + 3 can be recognized as a sum because it can be
broken into 1 + 2 , also a sum, and + 3 , a suitable suffix.

               1+2+3+4
               is an expression, 1+2, followed by suitable suffix,
               expr+3+4
               Which is
               expr + 4
               Which is
               expr

We control left/right associativity by selecting which side of the operator

recursion is performed

    BINARY_OP: "+" | "-" | "*" | "/"
    expr : expr BINARY_OP NUMBER
        | NUMBER

We control left/right associativity by selecting which side of the

operator recursion is performed.

If we turn it another way around, operators would be right-associative.

     BINARY_OP: "+" | "-" | "*" | "/"
     expr : NUMBER BINARY_OP expr
         | NUMBER

BINARY_OP: "+" | "-" | "\*" | "/"
expr : expr BINARY_OP NUMBER
| NUMBER

           9+3+4+6

BINARY_OP: "+" | "-" | "\*" | "/"
expr : NUMBER BINARY_OP expr
| NUMBER

           9+3+4+6

Grammar is ambiguous (and incorrect)

Consider we would like to create a grammar for basic arithmetic operations like

1+2+3 or 7/8+9–2

BINARY_OP: "+" | "-" | "\*" | "/"
expr : expr BINARY_OP expr
| NUMBER
Grammar is unambiguous but incorrect

           BINARY_OP: "+" | "-" | "*" | "/"
           expr : expr BINARY_OP
               | NUMBER

First, we need to select how many priority levels we have.

For now, there are two: \*/ before +-. For each level,

we introduce a separate expression type. As \*/ needs

to be computed before +-, we conclude that multiplicative

expression should be “under” additive.

ADD_OP: "+" | "-"
MUL_OP: "\*" | "/"

expr: add_expr

add_expr: add_expr ADD_OP mul_expr
| mul_expr

mul_expr: mul_expr MUL_OP NUMBER
| NUMBER
the exponentiation operator ^ will be right-associative and has the

highest priority.

ADD_OP: "+" | "-"
MUL_OP: "\*" | "/"
POWER_OP: "^"

expr: add_expr

add_expr: add_expr ADD_OP mul_expr
| mul_expr

mul_expr: mul_expr MUL_OP pow_expr
| pow_expr

pow_expr: NUMBER POWER_OP pow_expr
| NUMBER
unary minus for expressions like -2+3 or 7\*-5. we introduce a new

non-terminal: primary. Primary can be either a number or a unary

operator followed by a number.

ADD_OP: "+" | "-"
MUL_OP: "\*" | "/"
POWER_OP: "^"

expr: add_expr

add_expr: add_expr ADD_OP mul_expr
| mul_expr

mul_expr: mul_expr MUL_OP pow_expr
| pow_expr

pow_expr: primary POWER_OP pow_expr
| primary

primary: "-" NUMBER
| NUMBER
 By definition, an expression in parenthesis has the highest priority.

ADD_OP: "+" | "-"
MUL_OP: "\*" | "/"
POWER_OP: "^"

expr: add_expr
add_expr: add_expr ADD_OP mul_expr
| mul_expr
mul_expr: mul_expr MUL_OP pow_expr
| pow_expr
pow_expr: primary POWER_OP pow_expr
| primary
primary: "-" NUMBER
| NUMBER
| "(" expr ")"
