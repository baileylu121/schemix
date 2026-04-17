# CSU33071 Compiler Design

A compiler translates the code written in one language to some other language without changing the meaning of the program. It is also expected that a compiler should make the target code efficient and optimized in terms of time and space.

Compiler Design is a one-semester module taken by third year Integrated Computer Science students studying tools designed for writers of compilers and interpreters which are also useful for many other applications. The course is relevant for any application that looks for patterns in its input or has an input or command language.

## Timetable

- In person Lecture 10:00 Monday LB08
- In person Lecture 12:00 Wednesday LB01

## Assessment Structure

100% In person written 2 hour examination.
Supplemental exam 100% In person written 2 hour examination.

## Codemark

Codemark is a virtual learning environment and course management system.

- Register for CSU33071 Compiler Design — You need to provide your student number and choose a four digit PIN which you must not forget.
- Submit code for testing
- Submit archive file for testing
- Show all codemark results
- Forgot PIN

## Programming Assignments (Ungraded)

| #   | Assignment          | File                                                       | Guideline Submission |
| --- | ------------------- | ---------------------------------------------------------- | -------------------- |
| 1   | even.l              | [even.txt](Assignments/even.txt)                           | Week 2               |
| 2   | comments.l          | [comments.txt](Assignments/comments.txt)                   | Week 3               |
| 3   | plates.l            | [plates.txt](Assignments/plates.txt)                       | Week 5               |
| 4   | roman.y             | [roman.txt](Assignments/roman.txt)                         | Week 7               |
| 5   | romcalc.y           | [romcalc.txt](Assignments/romcalc.txt)                     | Week 8               |
| 6   | calcwithvariables.y | [calcwithvariables.txt](Assignments/calcwithvariables.txt) | Week 10              |

## Written Notes (PDFs)

| Topic                 | File                                                      |
| --------------------- | --------------------------------------------------------- |
| Introduction          | [Introduction.pdf](Lectures/Introduction.pdf)             |
| Lexical Analysis      | [Lexical.pdf](Lectures/Lexical.pdf)                       |
| RegExp                | [regexp.pdf](Lectures/regexp.pdf)                         |
| Calculator            | [calculator.pdf](Lectures/calculator.pdf)                 |
| RegLang               | [reglang.pdf](Lectures/reglang.pdf)                       |
| Bison                 | [bison.pdf](Lectures/bison.pdf)                           |
| Precedence            | [precedence.pdf](Lectures/precedence.pdf)                 |
| Context Free Grammars | [cfg.pdf](Lectures/cfg.pdf)                               |
| Abstract Syntax Tree  | [AbstractSyntaxTree.pdf](Lectures/AbstractSyntaxTree.pdf) |
| Advanced Calculator   | [advcal.pdf](Lectures/advcal.pdf)                         |
| Top Down Parsing      | [topdown.pdf](Lectures/topdown.pdf)                       |
| Predictive Parsing    | [predictive.pdf](Lectures/predictive.pdf)                 |
| Bottom Up Parsing     | [bottomup.pdf](Lectures/bottomup.pdf)                     |
| SR Conflicts (Q27)    | [sr.pdf](Lectures/sr.pdf)                                 |
| Context Free Grammar  | [ContextFreeGrammar.pdf](Lectures/ContextFreeGrammar.pdf) |

## Video Notes

| Date       | Topic               | File                                                                |
| ---------- | ------------------- | ------------------------------------------------------------------- |
| 2026-03-26 | Q27                 | [Q27-2025-03-26.mp4](Videos/Q27-2025-03-26.mp4)                     |
| 2026-03-25 | Bottom Up           | [BottomUp-2026-03-25.mp4](Videos/BottomUp-2026-03-25.mp4)           |
| 2026-03-23 | Deterministic       | [Deterministic-2026-03-23.mp4](Videos/Deterministic-2026-03-23.mp4) |
| 2026-03-18 | Recursion           | [Recursion-2026-03-18.mp4](Videos/Recursion-2026-03-18.mp4)         |
| 2026-03-16 | Advanced Calculator | [AdvCalc-2026-03-16.mp4](Videos/AdvCalc-2026-03-16.mp4)             |
| 2026-03-11 | Interpreter         | [Interpreter-2026-03-11.mp4](Videos/Interpreter-2026-03-11.mp4)     |
| 2026-03-09 | Syntax Tree         | [SyntaxTree-2026-03-09.mp4](Videos/SyntaxTree-2026-03-09.mp4)       |
| 2026-02-18 | ZEV                 | [ZEV-2026-02-18.mp4](Videos/ZEV-2026-02-18.mp4)                     |
| 2026-02-16 | CFG                 | [CFG-2026-02-16.mp4](Videos/CFG-2026-02-16.mp4)                     |
| 2026-02-11 | Bison               | [Bison-2026-02-11.mp4](Videos/Bison-2026-02-11.mp4)                 |
| 2026-02-09 | Parsing             | [Parsing-2026-02-09.mp4](Videos/Parsing-2026-02-09.mp4)             |
| 2026-02-04 | NFA                 | [NFA-2026-02-04.mp4](Videos/NFA-2026-02-04.mp4)                     |
| 2026-01-26 | Calculator          | [Calculator-2026-01-26.mp4](Videos/Calculator-2026-01-26.mp4)       |
| 2026-01-21 | RegExp              | [RegExp-2026-01-21.mp4](Videos/RegExp-2026-01-21.mp4)               |
| 2026-01-19 | Intro               | [Intro-2026-01-19.mp4](Videos/Intro-2026-01-19.mp4)                 |

## Example Materials

- [Example Exam](Example%20Exam.pdf) — CSU33071-Compiler-Design-Annual-2025.pdf
- [Answers](answers.txt)
- [Optical Mark Answer Sheet](Optical%20Mark%20Answer%20Sheet.pdf)

## Recommended Text

**Flex and Bison** by John Levine

- Publication date: 28 Aug 2009
- Publisher: O'Reilly Media, Inc, USA

![Flex and Bison book cover](Resources/fb.png)

## Resources

- [mac.txt](Files/mac.txt) — How to run flex on Mac
- [cfg.py](Files/cfg.py)
- [cfg2.py](Files/cfg2.py)
- [cfg3.py](Files/cfg3.py)
- [cfg4.py](Files/cfg4.py)
- [first.py](Files/first.py)
- [left.py](Files/left.py)
- [right.py](Files/right.py)
- [test.l](Files/test.l)
- [wrong.y](Files/wrong.y)
- [lr.png](Files/lr.png)
- [tree.png](Files/tree.png)

### Flex and Bison Examples

| File                               | Description      |
| ---------------------------------- | ---------------- |
| [fb1-1.l](Files/fb1-1.l)           | Flex example     |
| [fb1-3.l](Files/fb1-3.l)           | Flex example     |
| [fb1-4.l](Files/fb1-4.l)           | Flex example     |
| [fb1-5.l](Files/fb1-5.l)           | Flex example     |
| [fb1-5.y](Files/fb1-5.y)           | Bison grammar    |
| [fb1-5.tab.c](Files/fb1-5.tab.c)   | Generated parser |
| [fb1-5.tab.h](Files/fb1-5.tab.h)   | Generated header |
| [fb3-1.l](Files/fb3-1.l)           | Flex example     |
| [fb3-1.y](Files/fb3-1.y)           | Bison grammar    |
| [fb3-1.h](Files/fb3-1.h)           | Header           |
| [fb3-1funcs.c](Files/fb3-1funcs.c) | Functions        |
| [fb3-1.lex.c](Files/fb3-1.lex.c)   | Generated lexer  |
| [fb3-1.tab.c](Files/fb3-1.tab.c)   | Generated parser |
| [fb3-1.tab.h](Files/fb3-1.tab.h)   | Generated header |
| [fb3-2.l](Files/fb3-2.l)           | Flex example     |
| [fb3-2.y](Files/fb3-2.y)           | Bison grammar    |
| [fb3-2.h](Files/fb3-2.h)           | Header           |
| [fb3-2funcs.c](Files/fb3-2funcs.c) | Functions        |
| [fb3-2.tab.c](Files/fb3-2.tab.c)   | Generated parser |
| [fb3-2.tab.h](Files/fb3-2.tab.h)   | Generated header |
| [lex.yy.c](Files/lex.yy.c)         | Generated lexer  |

## FAQ

See [answers.txt](answers.txt) for frequently asked questions.

---

_Cloned from https://www.scss.tcd.ie/John.Waldron/CSU33071/CSU33071.html_
