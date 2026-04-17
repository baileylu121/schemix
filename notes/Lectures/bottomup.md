                          Language Analysis

We can analyse the sentences of a language by extracting the
grammatical structure of the sentence.

This is done by building an explicit or implicit parse tree
for the sentence.

The parser can be organized so as to attempt to build a
derivation of the sentence from the start symbol of the grammar.

This kind of parser is referred to as a top-down parser, because
in terms of the parse tree, it is built from the top to the bottom.

Alternately, it can be organized so as to attempt to trace out a
derivation in reverse, going from the sentence to the start symbol.

This kind of parser is referred to as a bottom-up parser, because
in terms of the parse tree, it is built from the bottom to the top.
A top-down parser needs to make two kinds of decisions at each step:

1/ It needs to choose a nonterminal in the frontier of the current
parse tree to expand next.

2/ In general, a nonterminal may have several rules associated with it.
Hence the parser needs to decide which rule to use to expand the
nonterminal it chose in (1).

A bottom-up parser needs to make similar decisions.

A common organization used in batch compilers is for the parser to
call the scanner whenever it is needs another token.

Usually, the parser needs to look at one or more tokens to make
its parsing decisions: these tokens which the parser is examining
but not yet consumed are called lookahead tokens.

Typically, the parser needs to use a single lookahead token.
 Shift-Reduce Parsing

A shift-reduce parser uses a parse stack which (conceptually) contains grammar symbols.

During the operation of the parser, symbols from the input are shifted onto the stack.

If a prefix of the symbols on top of the stack matches the RHS of a grammar rule which
is the correct rule to use within the current context, then the parser reduces the RHS of
the rule to its LHS, replacing the RHS symbols on top of the stack with the nonterminal
occurring on the LHS of the rule.

This shift-reduce process continues until the parser terminates, reporting either success
or failure. It terminates with success when the input is legal and is accepted by the parser.

It terminates with failure if an error is detected in the input.
The parser is a stack automaton which may be in one of several discrete
states.

A state is usually represented simply as an integer.

In reality, the parse stack contains states, rather than grammar symbols.

However, since each state corresponds to a unique grammar symbol, the state stack
can be mapped onto the grammar symbol stack mentioned earlier.
So, for example, since you'll never have a "s3" following on two different
symbols, you know when you're in state 3 that you've just seen a ":=",
just as to get to states 5, 9 or 10 you'll have to have just seen an ID.

In practice, bison also maintains a stack for attributes (the $1, $2
etc.), so when there's (conceptually) an ID on the stack, there might
actually be a "a" or "b" on the attribute stack.
The operation of the parser is controlled by a couple of tables:

Action Table

The action table is a table with rows indexed by states and columns indexed by
terminal symbols. When the parser is in some state s and the current lookahead
terminal is t, the action taken by the parser depends on the contents of action[s][t],
which can contain four different kinds of entries:
Shift s'
Shift state s' onto the parse stack.
Reduce r
Reduce by rule r. This is explained in more detail below.
Accept
Terminate the parse with success, accepting the input.
Error
Signal a parse error.
Goto Table

The goto table is a table with rows indexed by states and columns indexed by
nonterminal symbols.

When the parser is in state s immediately after reducing by rule N, then the next
state to enter is given by goto[s][N].
The current state of a shift-reduce parser is the state on top of the state stack.
The detailed operation of such a parser is as follows:

1 Initialize the parse stack to contain a single state s0, where s0 is the distinguished
initial state of the parser.
2 Use the state s on top of the parse stack and the current lookahead t to consult the
action table entry action[s][t]:
If the action table entry is shift s' then push state s' onto the stack and advance
the input so that the lookahead is set to the next token.
If the action table entry is reduce r and rule r has m symbols in its RHS, then
pop m symbols off the parse stack. Let s' be the state now revealed on top of the
parse stack and N be the LHS nonterminal for rule r.
Then consult the goto table and push the state given by goto[s'][N] onto the stack.
The lookahead token is not changed by this step.
If the action table entry is accept, then terminate the parse with success.
If the action table entry is error, then signal an error.
3 Repeat step (2) until the parser terminates.
For example, consider the following simple grammar

0.  $S: stmt <EOF>
1.  stmt: ID ':=' expr
2.  expr: expr '+' ID
3.  expr: expr '-' ID
4.  expr: ID

which describes assignment statements like a:= b + c - d

(Rule 0 is a special augmenting production added to the grammar).
sn denotes shift n,
rn denotes reduce n, input a:= b + c - d
acc denotes accept
blank entries denote error entries

                                     0)   $S: stmt <EOF>
                                     1)   stmt: ID ':=' expr
                                     2)   expr: expr '+' ID
                                     3)   expr: expr '-' ID
                                     4)   expr: ID

1 Initialize the parse stack to contain a single state s0, where s0 is the distinguished
initial state of the parser.
 input a := b + c - d
1/

     Each stack entry is shown as a state number
     followed
     by the symbol which caused the transition to
     that state.

Use the state s on top of the parse stack and the current lookahead t to consult the
action table entry action[s][t]:
If the action table entry is shift s' then push state s' onto the stack and advance
the input so that the lookahead is set to the next token.
input a := b + c - d
 2/ input a := b + c - d

Use the state s on top of the parse stack and the current lookahead t to consult the
action table entry action[s][t]:
If the action table entry is shift s' then push state s' onto the stack and advance
the input so that the lookahead is set to the next token.
 3/
input a := b + c - d

Use the state s on top of the parse stack and the current lookahead t to consult the
action table entry action[s][t]:
If the action table entry is shift s' then push state s' onto the stack and advance
the input so that the lookahead is set to the next token.
 input a := b + c - d

                                                    0)   $S: stmt <EOF>
                                                    1)   stmt: ID ':=' expr
                                                    2)   expr: expr '+' ID
                                                    3)   expr: expr '-' ID
                                                    4)   expr: ID

If the action table entry is reduce r and rule r has m symbols in its RHS, then
pop m symbols off the parse stack. Let s' be the state now revealed on top of the
parse stack and N be the LHS nonterminal for rule r.
Then consult the goto table and push the state given by goto[s'][N] onto the stack.
The lookahead token is not changed by this step.
 input a := b + c - d

                                                               0)   $S: stmt <EOF>
                                                               1)   stmt: ID ':=' expr
                                                               2)   expr: expr '+' ID
                                                               3)   expr: expr '-' ID
                                                               4)   expr: ID

If the action table entry is reduce r and rule r has m symbols in its RHS, then
pop m symbols off the parse stack. Let s' be the state now revealed on top of the
parse stack and N be the LHS nonterminal for rule r.
Then consult the goto table and push the state given by goto[s'][N] onto the stack.
The lookahead token is not changed by this step.
 input a := b + c - d

If the action table entry is reduce r and rule r has m symbols in its RHS, then
pop m symbols off the parse stack. Let s' be the state now revealed on top of the
parse stack and N be the LHS nonterminal for rule r.
Then consult the goto table and push the state given by goto[s'][N] onto the stack.
The lookahead token is not changed by this step.
 input a := b + c - d

                                                   0)   $S: stmt <EOF>
                                                   1)   stmt: ID ':=' expr
                                                   2)   expr: expr '+' ID
                                                   3)   expr: expr '-' ID
                                                   4)   expr: ID

If the action table entry is reduce r and rule r has m symbols in its RHS, then
pop m symbols off the parse stack. Let s' be the state now revealed on top of the
parse stack and N be the LHS nonterminal for rule r.
Then consult the goto table and push the state given by goto[s'][N] onto the stack.
The lookahead token is not changed by this step.
 input a := b + c - d

0.  $S: stmt <EOF>
1.  stmt: ID ':=' expr
2.  expr: expr '+' ID
3.  expr: expr '-' ID
4.  expr: ID
     input a := b + c - d

5.  $S: stmt <EOF>
6.  stmt: ID ':=' expr
7.  expr: expr '+' ID
8.  expr: expr '-' ID
9.  expr: ID
    input a := b + c - d
     input a := b + c - d

If the action table entry is reduce r and rule r has m symbols in its RHS, then
pop m symbols off the parse stack. Let s' be the state now revealed on top of the
parse stack and N be the LHS nonterminal for rule r.
Then consult the goto table and push the state given by goto[s'][N] onto the stack.
The lookahead token is not changed by this step.
input a := b + c - d

        0)   $S: stmt <EOF>
        1)   stmt: ID ':=' expr
        2)   expr: expr '+' ID
        3)   expr: expr '-' ID
        4)   expr: ID

input a := b + c - d
 input a:= b + c - d

                                  0)   $S: stmt <EOF>
               $S                 1)   stmt: ID ':=' expr
                                  2)   expr: expr '+' ID
        stmt        EOF           3)   expr: expr '-' ID
                                  4)   expr: ID

ID a := expr

          expr
                    -      ID d

expr + ID c

ID b
 Construction of Shift-Reduce Parsing Tables

The general idea of bottom-up parsing is to repeatedly match the RHS of some rule
and reduce it to the rule's LHS.

To identify the matching RHS's, the parser needs to keep track of all possible rules
which may match.

This is done by means of the parser state, where each state keeps track of the set of
rules the parser may currently be in, and how far along the parser may be within each
rule.

This idea of states will become clearer if we attempt to build the tables for a small
example.
Consider the grammar

0.  $S: stmt <EOF>
1.  stmt: ID ':=' expr
2.  expr: expr '+' ID
3.  expr: expr '-' ID
4.  expr: ID

The input must be ultimately reducible to the augmenting nonterminal $S.

Hence the parser should initially be in rule 0; more specifically, it should be expecting
the stmt in rule 0.
To show precisely which symbol is expected in a rule RHS, we define an item to be a
rule, along with a position on the RHS specifying the next symbol to be expected in
that RHS.

We denote an item as a rule with a dot . just before the next expected symbol.
Hence, returning to our example, the parser is initially expecting the item

0. $S: . stmt <EOF>

However, if the parser is expecting to see a stmt, it could be at the beginning of any of
the rules for stmt.
Hence the initial state should include the initial item for stmt. (The process of including
these additional induced items is referred to as forming the closure of the state).

State 0 0) $S: . stmt <EOF>

1. stmt: . ID ':=' expr
   Now if the parser sees an ID in state 0, then it can move the dot past any ID symbols
   in state 0. We get a new state; let's call it State 1:

State 1

1. stmt: ID . ':=' expr

If the parser has seen a stmt in state 0, then it can move the dot past any stmt
symbols in state 0. We get a new state; let's call it State 2:

State 2 0) $S: stmt . <EOF>
However since the dot is before the nonterminal expr, the parser could be in any of the
rules for expr. Hence we need to include the rules for expr in a new state 3:

State 3

1. stmt: ID ':=' . expr
2. expr: . expr '+' ID
3. expr: . expr '-' ID
4. expr: . ID 0) $S: stmt <EOF> 1) stmt: ID ':=' expr 2) expr: expr '+' ID 3) expr: expr '-' ID 4) expr: ID
   We continue this process of following all possible transitions out of
   states until we cannot construct any new states.

The transitions on terminal symbols correspond to shift actions in the parser;
the transitions on nonterminal symbols correspond to goto actions in the parser.

Note that the construction guarantees that each state is entered by a
unique grammar symbol; that is why we can map a state stack into a
symbol stack as mentioned earlier.
0) $S: stmt <EOF>

1.  stmt: ID ':=' expr
2.  expr: expr '+' ID
3.  expr: expr '-' ID
4.  expr: ID
