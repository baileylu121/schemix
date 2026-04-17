                                                                              CSU33071-1




                Faculty of Engineering, Mathematics and Science
                     School of Computer Science & Statistics

Integrated Computer Science Programme Semester 2 2025
Year 3 Annual Examinations

                           CSU33071 – Compiler Design 1

??, ??????th ?????? 2025 Unknown Hall ??:?? – ??:??

                                  Prof. John Waldron

Instructions to Candidates:

Attempt as many questions as you can in the time available. The total marks on the
paper is 100 marks. Enter your answers on the CSU33071 Optical Mark Recognition
Answer Sheet provided. Fill in the circle corresponding to your answer for each question
•, preferably using a black pen. Do not write anything else anywhere on the sheet. Do
not make holes in the sheet or fold it. Place it inside the front cover of your answer book.
If you make a mistake the Invigilator can provide you with a spare sheet. Mark a large
X through your old page. You may not start this examination until you are instructed
to do so by the Invigilator. Exam Paper is not to be removed from venue.

Instructions for Invigilators:

To be accompanied by a CSU33071 Optical Mark Recognition Answer Sheet. Non-
programmable calculators are permitted for this examination.

                                     Page 1 of 14
                © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

In the following six questions the test cases are listed on one or more
lines separated by space characters. The space characters are not part
of the test cases.

Note: zero is 0, uppercase letter is O, lowercase is o

Q 1.
A correct answer is worth 2 marks, an incorrect answer loses 0.40 marks.

How many of the following 6 strings

ccc cc ccccc c cccc cccccc

are accepted, in part or whole, by the Thompson’s construction
nondeterministic finite state automaton shown below

                         c           c                c
                 0               1            2               3

(A) 5 (B) 4 (C) 1 (D) 6 (E) 3 (F) OTHER

Q 2.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 15 strings

llllzzzlll zzzzz yyyyyyyzz lllzzyyyy yyyyyl lllyyylll zyyyyz
lllzzzyyyy yllllzzz yyylllzzzz yyyyyyyyyy yyyyyyy zzlll zlllyyy
yyyyyyyy

are accepted, in part or whole, by the Thompson’s construction
nondeterministic finite state automaton shown below

                                          l
                                 ε   2            3       ε
                     z
             0               1   ε                        ε       6
                                          y
                                     4            5

(A) 6 (B) 5 (C) 13 (D) 14 (E) 15 (F) OTHER

                               Page 2 of 14
          © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 3.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 15 strings

tttttttttt hhhvvh vhhhhv vvvvhhh tttthhvvvv vvvvvtttt tvvvvt
vvvvhhhhvvvv hhhhhhh ttttvvvtttt ttthhttt tthv vvvtvvvv hhhhvhhhh
vvvtttvvvv

are accepted, in part or whole, by the Thompson’s construction
nondeterministic finite state automaton shown below

                                 v
                     ε       1       2      ε
                                                            t
             0       ε                      ε       5               6
                                 h
                             3       4

(A) 14 (B) 15 (C) 10 (D) 2 (E) 6 (F) OTHER

Q 4.
A correct answer is worth 2 marks, an incorrect answer loses 0.40 marks.

How many of the following 15 strings

yyyddduu ddduuuddd yyyydddddd dddddduuu dyyy dddddddddd ddddyyuuuu
uuyyyy uuddddy uuuuu duuuuuu yyyyyyyyuuuu uudddduuuu yyyyyyuuuu
dddduuuuu

are accepted, in part or whole, by the Thompson’s construction
nondeterministic finite state automaton shown below

                         d           y                  u
                 0               1              2               3

(A) 0 (B) 8 (C) 3 (D) 15 (E) 12 (F) OTHER

                               Page 3 of 14
          © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 5.
A correct answer is worth 4 marks, an incorrect answer loses 0.80 marks.

How many of the following 15 strings

gggxxhhhh gxxxxhh ggxxxxhhh gxhh gggxhhhh ggggxhhhh ggggxxxxh gxhhh
gxxxhh ggxxxhh gggxxxxhhhh ggxhhhh ggggxxxhhh gggxxh gggxxxhh

are accepted, in part or whole, by the Thompson’s construction
nondeterministic finite state automaton shown below

                                                     g
                                             2           3

                                     ε                           ε
                                                     x
                                     ε       4           5       ε
                             1                       ε
                                                                         6
                     ε                                                           ε
                                                                                                 h
             0                                                                           7               8
                                                     ε

(A) 12 (B) 14 (C) 9 (D) 15 (E) 8 (F) OTHER

Q 6.
A correct answer is worth 4 marks, an incorrect answer loses 0.80 marks.

How many of the following 15 strings

arrrraaaa aaaaar rrrffffaaaa aarrfff ffaaa rrrrfff fffaa rrraarr
rrrrrraaa aaaarrrrrr aaarrrrrr ffffrrrrrrrr aaaaaaaa rrrff aaaarrrraa

are accepted, in part or whole, by the Thompson’s construction
nondeterministic finite state automaton shown below

                                                         f
                                                     3       4

                                                 ε                   ε
                                                         a
                                                 ε   5       6       ε
                                         2               ε
                                                                             7
                                 ε                                                   ε
                 r                                                                                   f
         0               1                                                                   8           9
                                                         ε

(A) 4 (B) 12 (C) 15 (D) 11 (E) 13 (F) OTHER

                                  Page 4 of 14
             © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

In the following twenty questions the test cases are listed on one or more
lines separated by space characters. The space characters are not part
of the test cases. Each test case will have a newline character \n
appended at the end, which will be matched by a $ symbol in a regular
expression.

Note: zero is 0, uppercase letter is O, lowercase is o

Q 7.
A correct answer is worth 2 marks, an incorrect answer loses 0.40 marks.

How many of the following 15 strings

JJJJ fJJJJJ JJJppp JJffJ JJfffppp ffpJ fffpppf pJJJJ JJJJJJppp
ppffp ffJJp ppffff ppJJJf pppf JJJJJp

are matched at least once, in part or whole, by the Flex regular expression

p[a-zA-Z]J

(A) 6 (B) 9 (C) 8 (D) 4 (E) 2 (F) OTHER

Q 8.
A correct answer is worth 2 marks, an incorrect answer loses 0.40 marks.

How many of the following 6 strings

VVVVVV VVVV V VVV VVVVV VV

are matched at least once, in part or whole, by the Flex regular expression

VVV

(A) 6 (B) 5 (C) 1 (D) 4 (E) 2 (F) OTHER

                              Page 5 of 14
         © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 9.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 15 strings

tnnttt FFFFt tnnnF FFFFFF nnnFF FFFnnnF nnttt nnntF nnnnnn FFnnn
nnFFF ntttttt tttttFF nFFFF FnF

are matched at least once, in part or whole, by the Flex regular expression

n[^A-Z]F

(A) 3 (B) 8 (C) 5 (D) 7 (E) 13 (F) OTHER

Q 10.

A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 15 strings

lllTx lllllx lllTxx lxxxT lTTTxxx llllllx xTTTx xxxllxxx TTTTxxx
TTTxxxx TTlllxx TTTx lxxlll xxxlT TTTxx

are matched at least once, in part or whole, by the Flex regular expression

x[a-zA-Z][a-zA-Z]?ll

(A) 4 (B) 6 (C) 2 (D) 5 (E) 10 (F) OTHER

Q 11.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 15 strings

vIIv vII Ivii viiII IIIIv vvvv iiII ivii ivII IIiii IIIv iivvI iiI
IiiI vvIi

are matched at least once, in part or whole, by the Flex regular expression

(iii|vv)

(A) 7 (B) 14 (C) 11 (D) 4 (E) 2 (F) OTHER

                                Page 6 of 14
           © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 12.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 15 strings

CClllll CCCC lllllCC ddddd CCCdddd lllllC CllC lCC CClllCC ldddddd
CCdddd CCCCC ddCd dCCClll llldCC

are matched at least once, in part or whole, by the Flex regular expression

d[^a-z]C

(A) 8 (B) 2 (C) 12 (D) 7 (E) 9 (F) OTHER

Q 13.

A correct answer is worth 4 marks, an incorrect answer loses 0.80 marks.

How many of the following 15 strings

QaaQ ssa aaaass QQQQss QQaass Qssa ssQQss QQQQQ QQaaQQ saas aaQQ
ssaQ aaaaaa sQa QQss

are matched at least once, in part or whole, by the Flex regular expression

(a{2,3}|sss?)

(A) 14 (B) 4 (C) 10 (D) 2 (E) 13 (F) OTHER

Q 14.
A correct answer is worth 4 marks, an incorrect answer loses 0.80 marks.

How many of the following 15 strings

kkkk kkii iiDDkk kiik iikkDD Diii ikkkk DDik kkkkD kii kDii Dkki
iiiD kkiikk kkiii

are matched at least once, in part or whole, by the Flex regular expression

(i{2}|D{1,2}|[A-M]+)$

(A) 5 (B) 6 (C) 8 (D) 13 (E) 9 (F) OTHER

                                Page 7 of 14
           © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 15.
A correct answer is worth 4 marks, an incorrect answer loses 0.80 marks.

How many of the following 15 strings

yyJJ yddyy yJJyy dddJJ dyydd dyd yyyd yyyyJJ yyyydd JJddJJ ddddd
Jyyyy dJJyy dyyyy ydd

are matched at least once, in part or whole, by the Flex regular expression

[a-z][A-Z]$

(A) 7 (B) 1 (C) 0 (D) 5 (E) 2 (F) OTHER

Q 16.

A correct answer is worth 4 marks, an incorrect answer loses 0.80 marks.

How many of the following 15 strings

Qvvvvv QQQssss sssvvvvv QQQvvQQ QQQQvvv QssQQQ vvvvvvss QQQvss
QQQQQv vssss vvsv QvvvQQ vvvvvvv vvvssv QQQssQQ

are matched at least once, in part or whole, by the Flex regular expression

..vQ.

(A) 8 (B) 15 (C) 2 (D) 7 (E) 14 (F) OTHER

                              Page 8 of 14
         © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 17.
A correct answer is worth 2 marks, an incorrect answer loses 0.40 marks.

How many of the following 7 sentences

EEEE EEE E EEEEEE sentence EE mRpF5Hu

are in the language defined by the Bison Context Free Grammar

%token E
%%
sentence: E | E sentence
;

(A) 4 (B) 2 (C) 7 (D) 1 (E) 5 (F) OTHER

Q 18.
A correct answer is worth 2 marks, an incorrect answer loses 0.40 marks.

How many of the following 6 sentences

BKHDqiC IIIII IIII I II sentence

are in the language defined by the Bison Context Free Grammar

%token I
%%
sentence: I | sentence I
;

(A) 1 (B) 4 (C) 5 (D) 2 (E) 3 (F) OTHER

                              Page 9 of 14
         © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 19.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 7 sentences

sentence dddOOO sSMXMh1 dddddddOOOOO ddddddOOO ddddddO ddOOO

are in the language defined by the Bison Context Free Grammar

%token d O
%%
sentence: sub | sub sentence
sub: d | O
;

(A) 1 (B) 5 (C) 3 (D) 4 (E) 6 (F) OTHER

Q 20.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 10 sentences

sssH ssHHHH sHHH sHHHH sssHHH ssHHH ssssHHHH sss sssHHHH HH

are in the language defined by the Bison Context Free Grammar

%token s H
%%
sentence: s | H | s sentence
;

(A) 2 (B) 9 (C) 8 (D) 7 (E) 1 (F) OTHER

                             Page 10 of 14
         © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 21.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 10 sentences

zGG zzzGG zG GG zGGG zzzGGGG zzGGG zzzzGGGG zz zzGG

are in the language defined by the Bison Context Free Grammar

%token z G
%%
sentence: z | G | sentence z
;

(A) 10 (B) 8 (C) 1 (D) 7 (E) 2 (F) OTHER

Q 22.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 10 sentences

xxxA xxAAAA xxxxAA xxxx A xxxAA xAAAA xxxxAAAA xxAAA xxAA

are in the language defined by the Bison Context Free Grammar

%token x A
%%
sentence: x | A | A sentence
;

(A) 5 (B) 1 (C) 3 (D) 8 (E) 7 (F) OTHER

                             Page 11 of 14
         © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 23.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 10 sentences

ssssBBB sBBBB sssBBB ssss ssB ssBB sB sBB BB sssBB

are in the language defined by the Bison Context Free Grammar

%token s B
%%
sentence: s | B | sentence B
;

(A) 3 (B) 5 (C) 4 (D) 9 (E) 7 (F) OTHER

Q 24.
A correct answer is worth 3 marks, an incorrect answer loses 0.60 marks.

How many of the following 6 sentences

II;IIII;III IIII;III;II IIIIIII;II;II IIIIII;IIIIII I;IIIII IIIIIIIII

are in the language defined by the Bison Context Free Grammar

%token I
%%
sentence: list | sentence list
list: listc ’;’
listc: I | I listc
;

(A) 2 (B) 4 (C) 0 (D) 1 (E) 3 (F) OTHER

                             Page 12 of 14
         © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 25.
A correct answer is worth 4 marks, an incorrect answer loses 0.80 marks.

How many of the following 5 sentences

MMMMMMM,MMMMM, MM,MMMMMMMMM, MMM,M,MMM M,MMMMMMMM,MM MMM,M,MM,MMM,

are in the language defined by the Bison Context Free Grammar

%token M
%%
sentence: listc | listc ’,’ sentence
listc: M | M listc
;

(A) 2 (B) 4 (C) 3 (D) 1 (E) 5 (F) OTHER

Q 26.
A correct answer is worth 4 marks, an incorrect answer loses 0.80 marks.

How many of the following 7 sentences

lll,l,llll,ll llll,lll,llllllll,llll, llllllllll; lllllllll,lllllll
llll,llllll,l; ll,ll lllllllll;

are in the language defined by the Bison Context Free Grammar

%token l
%%
sentence: commal ’;’
commal: listc | listc ’,’ commal
listc: l | l listc
;

(A) 1 (B) 7 (C) 5 (D) 4 (E) 3 (F) OTHER

                             Page 13 of 14
         © Trinity College Dublin, The University of Dublin 2025

CSU33071-1

Q 27.
A fully correct answer is worth 20 marks.

Given the following tokens
"^" { return POWER; }
"-" { return MINUS; }
":=" { return ASSIGN; }
[a-z] { yylval = yytext[0]; return ID; }
\n { return EOL; }
and the following Bison Context Free Grammar
0 $accept: S $end
1 S: stmt EOL
2 stmt: ID ASSIGN expr
3 expr: unary POWER expr
4 | unary
5 unary: MINUS unary
6 | ID
which generates the Bison Shift Reduce Parser

                                                                                                      State 0

                                                                                                  0 $accept: • S $end

                                                                                             ID           S             stmt

                                                               State 1                                State 2                  State 3

                                                      2 stmt: ID • ASSIGN expr                    0 $accept: S • $end      1 S: stmt • EOL

                                                          ASSIGN                                          $end                    EOL

                                        State 4                                                       State 5                  State 6

                               2 stmt: ID ASSIGN • expr                                           0 $accept: S $end •      1 S: stmt EOL •

                                              unary                expr

                                        State 10
                                                                                   State 9
                                                                                                                 Acc             R1
                             3 expr: unary • POWER expr
                                                                         2 stmt: ID ASSIGN expr •
                             4     | unary •

                    MINUS                      unary    POWER

                                                       State 12
        ID                   R4                                                         R2
                                          3 expr: unary POWER • expr

                                              MINUS                       expr

                     State 8                                                     State 13
                                               MINUS      ID
              5 unary: MINUS • unary                                3 expr: unary POWER expr •

                    unary         ID

         State 11                      State 7
                                                                                   R3
    5 unary: MINUS unary •        6 unary: ID •



             R5                          R6

What sequence of states will the Bison Shift Reduce Parser go through
parsing the sentence
a:=b^c\n

                                 Page 14 of 14
             © Trinity College Dublin, The University of Dublin 2025
