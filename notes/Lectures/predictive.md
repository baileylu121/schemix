                                                                     always k=1

A deterministic model of computation is a model of computation such that the
successive states of the machine and the operations to be performed are
completely determined by the preceding state.
LL(1) grammars cannot be left recursive since the leftmost

nonterminal is the same as the LHS. This would result in

infinite recursion.
Left factoring is removing the common left factor that appears
in two productions of the same non-terminal.

It is done to avoid back-tracing by the parser.

Suppose the parser has a look-ahead consider this example

A -> qB | qC
where A,B,C are non-terminals and q is a sentence. In this case,
the parser will be confused as to which of the two productions
to choose and it might have to back-trace.
After left factoring, the grammar is converted to

A -> qD

D -> B | C

In this case, a parser with a look-ahead will always choose the
right production.
common prefix

    suffixes

Parsing table

non-terminal

                         terminal on top of stack

terminal matches input, pop and advance
input
$ marks end of input
