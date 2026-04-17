Recursive Descent Parsing Algorithm – top down parsing.
4+5+6\*7
(5)
grammar

          Parse tree

input
advances next, returns boolean
 these advance next

&& evaluates arguments in left to right order
|| if first branch succeeds, do not bother with second branch

                             backtracking

if they all fail, the higher level will do the backtracking
In the formal language theory of computer science, left recursion
is a special case of recursion where a string is recognized as
part of a language by the fact that it decomposes into a
string from that same language (on the left) and a suffix
(on the right).
A limitation of recursive descent

                                    int * int will be rejected




                                    once a non terminal succeeds
                                    no way to try another production
