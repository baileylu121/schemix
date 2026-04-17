# Schemix — Type-Def DSL to Nix Module Options Compiler

## Overview

A compiler that takes a type-definition DSL and produces Nix module option declarations. Written in Rust using **Chumsky** (parser combinators) and **Ariadne** (span-based error reporting). The project doubles as a compiler-design revision exercise mapped to the CSU33071 course content.

**Implementer note**: This plan is structured to follow the course progression linearly. Each phase maps to lecture topics and builds on the previous one. Implement phases in order.

---

## 1. The DSL: Schemix Syntax

### 1.1 Example Input

```schemix
// A web server configuration module
module services.nginx {
    enable: bool "Whether to enable nginx" = false,
    port: int "Port to listen on" = 80,
    server_name: str "Server hostname" = "localhost",
    root: path "Document root",
    extra_config: attrsOf str "Additional config key-values",
    vhosts: listOf (submodule {
        name: str "Virtual host name",
        root: path "Document root",
        ssl: bool "Enable TLS" = false,
    }),
    log_level: enum ["debug" "info" "warn" "error"] "Log verbosity" = "warn",
    @readonly bind_address: str "Bind address" = "0.0.0.0",
}
```

### 1.2 Type System

| DSL Type            | Nix Output Type                          | Category            |
| ------------------- | ---------------------------------------- | ------------------- |
| `bool`              | `lib.types.bool`                         | Primitive           |
| `int`               | `lib.types.int`                          | Primitive           |
| `str`               | `lib.types.str`                          | Primitive           |
| `path`              | `lib.types.path`                         | Primitive           |
| `anything`          | `lib.types.anything`                     | Primitive           |
| `package`           | `lib.types.package`                      | Primitive           |
| `listOf T`          | `lib.types.listOf T`                     | Parametric          |
| `attrsOf T`         | `lib.types.attrsOf T`                    | Parametric          |
| `nullOr T`          | `lib.types.nullOr T`                     | Parametric          |
| `functionTo T`      | `lib.types.functionTo T`                 | Parametric          |
| `submodule { ... }` | `lib.types.submodule { options = ...; }` | Recursive composite |
| `enum [a b c]`      | `lib.types.enum [a b c]`                 | Composite           |

### 1.3 Option Syntax (full form)

```
@visibility name: Type "description" = default | example
```

All parts after the name and type are optional:

- Description: a double-quoted string
- Default: `=` followed by a literal (int, string, bool)
- Example: `|` followed by a literal
- Visibility: `@internal` or `@readonly` prefix, can stack

### 1.4 Module Syntax

```
module <dotted.path> {
    <options separated by commas>
}
```

A file can contain multiple `module` blocks. Trailing comma is allowed.

### 1.5 Comments

- Line comments: `// ...`
- Block comments: `/* ... */`

### 1.6 Example Output

For the input above, the compiler emits:

```nix
{ lib, ... }:

{
  options.services.nginx = {
    enable = lib.mkOption {
      type = lib.types.bool;
      description = "Whether to enable nginx";
      default = false;
    };

    port = lib.mkOption {
      type = lib.types.int;
      description = "Port to listen on";
      default = 80;
    };

    server_name = lib.mkOption {
      type = lib.types.str;
      description = "Server hostname";
      default = "localhost";
    };

    root = lib.mkOption {
      type = lib.types.path;
      description = "Document root";
    };

    extra_config = lib.mkOption {
      type = lib.types.attrsOf lib.types.str;
      description = "Additional config key-values";
    };

    vhosts = lib.mkOption {
      type = lib.types.listOf (lib.types.submodule {
        options = {
          name = lib.mkOption {
            type = lib.types.str;
            description = "Virtual host name";
          };
          root = lib.mkOption {
            type = lib.types.path;
            description = "Document root";
          };
          ssl = lib.mkOption {
            type = lib.types.bool;
            description = "Enable TLS";
            default = false;
          };
        };
      });
      description = "";
    };

    log_level = lib.mkOption {
      type = lib.types.enum [ "debug" "info" "warn" "error" ];
      description = "Log verbosity";
      default = "warn";
    };

    bind_address = lib.mkOption {
      type = lib.types.str;
      description = "Bind address";
      default = "0.0.0.0";
      readOnly = true;
    };
  };
}
```

---

## 2. Implementation Phases (Linear, Course-Aligned)

### Phase 1: Project Skeleton + Understanding the Pipeline

**Course topics**: Introduction, Compiler Pipeline

Set up the project structure. Before writing any code, make sure you can articulate the compiler pipeline:

```
Source → Lexer → Tokens → Parser → AST → Semantic Analysis → Codegen → .nix
```

**Tasks**:

1. Update `Cargo.toml` with dependencies:
   ```toml
   [dependencies]
   chumsky = "0.10"
   ariadne = "0.5"
   clap = { version = "4", features = ["derive"] }
   ```
2. Create the source file skeleton:
   ```
   src/
     main.rs       — CLI entry, file I/O orchestration
     lexer.rs      — Token enum + Chumsky lexer
     ast.rs        — AST types
     parser.rs     — Chumsky parser (tokens → AST)
     semantic.rs   — Validation pass
     codegen.rs    — AST → Nix string
     error.rs      — Error types + Ariadne formatting
     span.rs       — Span/SourceId types
   ```
3. Wire up `main.rs` with clap: `schemix <input.sx> [-o <output.nix>]`
4. Make it compile (empty modules, stub functions) and print "not yet implemented"

**Deliverable**: `cargo build` succeeds. Running `schemix test.sx` prints a stub message.

---

### Phase 2: Lexical Analysis

**Course topics**: Lexical Analysis, Regular Expressions, Regular Languages, NFA/DFA, Thompson's Construction, Subset Construction

This is the scanner. It converts a character stream into a token stream.

**Concepts to revise while implementing**:

- **Regular expressions**: Each token is defined by a regex pattern. The Chumsky `text()` combinators are the equivalent of Flex patterns.
- **Longest match / rule priority**: Chumsky handles disambiguation internally. Understand _how_ it does this — it's the same principle as Flex's "longest match, then first rule".
- **NFA → DFA**: Chumsky compiles patterns into an efficient internal representation. You should be able to explain Thompson's construction (regex → NFA) and subset construction (NFA → DFA) even though Chumsky does this for you.
- **Scanning vs parsing**: The boundary between lexical analysis and syntax analysis. Where does lexing end and parsing begin?

**Token definitions**:

```rust
#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Keywords
    Module,
    Submodule,

    // Primitive types (also keywords)
    Bool, Int, Str, Path, Anything, Package,

    // Parametric type constructors
    ListOf, AttrsOf, NullOr, FunctionTo, Enum,

    // Visibility
    AtInternal,
    AtReadOnly,

    // Punctuation
    LBrace, RBrace,     // { }
    LParen, RParen,     // ( )
    LBracket, RBracket, // [ ]
    Comma,              // ,
    Colon,              // :
    Equals,             // =
    Pipe,               // |
    Dot,                // .

    // Literals
    IntLit(i64),
    StrLit(String),
    BoolLit(bool),

    // Identifiers
    Ident(String),
    DottedName(String),  // e.g. services.nginx

    // Special
    Newline,
}
```

**Key decisions**:

- `DottedName` vs `Ident . Ident . Ident`: Lex as a single token. The dot is part of the name, not a separate operator. This simplifies parsing and matches how Nix module paths work.
- `listOf`, `attrsOf`, etc. are keywords, not identifiers. Chumsky's ordering (keyword patterns before identifier pattern) implements the "first rule wins" disambiguation.
- String literals: double-quoted only. No Nix-style multi-line strings for v1.
- `@internal` / `@readonly`: Lex as single tokens (the `@` is part of the keyword).

**Tasks**:

1. Define `Token` enum with `PartialEq` for testing
2. Implement Chumsky lexer using `chumsky::text::` combinators
3. Handle whitespace and comments (strip them, don't produce tokens)
4. Store span information (byte offset) in each token for Ariadne
5. Write unit tests for each token type
6. Wire lexer into `main.rs`: read file → lex → print tokens (for now)
7. Wire Ariadne for lex errors: unterminated strings, invalid characters

**Testing**:

- Unit: each token type tokenizes correctly
- Property: any valid input produces only valid tokens
- Error: invalid inputs (unterminated strings, unknown characters) produce Ariadne errors

**Deliverable**: `schemix test.sx` prints the token stream. Lex errors display nicely with Ariadne.

---

### Phase 3: Context-Free Grammars & Parsing

**Course topics**: Context-Free Grammars, BNF, Bison, Precedence, Associativity, Top-Down Parsing, Recursive Descent, Predictive Parsing, Bottom-Up Parsing, Shift-Reduce

This is the parser. It converts a token stream into an AST.

**Concepts to revise while implementing**:

- **CFG / BNF**: The grammar below IS a context-free grammar. Each production rule (`<type_expr> ::= ...`) maps to a Chumsky combinator.
- **Parse trees vs AST**: Chumsky builds a parse tree implicitly; your semantic actions extract the AST. The AST drops punctuation, keywords, grouping — just like the course describes.
- **Left recursion**: Chumsky handles left-recursive grammars via `recursive()`. Understand why traditional recursive descent _can't_ handle left recursion, and how Chumsky's approach differs.
- **Associativity**: Controlled by the grammar structure. `listOf(attrsOf(int))` — inner types bind tighter.
- **Precedence**: The grammar's structure encodes precedence. Submodule is "tightest" (appears deepest), primitive types are "loosest".
- **Recursive descent**: Chumsky generates a recursive-descent parser. You should be able to hand-write the equivalent.
- **LL(1) / predictive parsing**: Chumsky uses arbitrary lookahead. Understand the difference between LL(1), LL(k), and PEG-style parsing.
- **Shift-reduce parsing**: Understand how Bison's LALR(1) approach differs from Chumsky's recursive-descent approach. Same language class, different algorithm.

**Grammar (BNF)**:

```
<program>       ::= <module_def>*

<module_def>    ::= "module" <dotted_name> "{" <option_list> "}"

<dotted_name>   ::= <ident> ("." <ident>)*

<option_list>   ::= "" | <option> ("," <option>)* ","?

<option>        ::= <visibility>* <ident> ":" <type_expr>
                      <string_lit>? <default_part>? <example_part>?

<visibility>    ::= "@internal" | "@readonly"

<type_expr>     ::= <primitive_type>
                  | "listOf" "(" <type_expr> ")"
                  | "attrsOf" "(" <type_expr> ")"
                  | "nullOr" "(" <type_expr> ")"
                  | "functionTo" "(" <type_expr> ")"
                  | "submodule" "{" <option_list> "}"
                  | "enum" "[" <string_list> "]"

<primitive_type>::= "bool" | "int" | "str" | "path"
                  | "anything" | "package"

<string_list>   ::= "" | <string_lit> (<string_lit>)*

<default_part>  ::= "=" <literal>

<example_part>  ::= "|" <literal>

<literal>       ::= <int_lit> | <string_lit> | <bool_lit>
```

**Tasks**:

1. Define AST types (see Phase 4)
2. Implement Chumsky parser using `chumsky::recursive()` for the type_expr (it's recursive via submodule)
3. Handle the `option_list` with trailing comma
4. Wire parser into `main.rs`: lex → parse → print AST (debug format for now)
5. Wire Ariadne for parse errors: "expected X, found Y" with spans
6. Write unit tests for each grammar production
7. Write integration tests for complete module definitions

**Testing**:

- Unit: each production parses correctly
- Integration: complete modules parse to correct ASTs
- Error: invalid syntax produces clear Ariadne errors
- Property: round-trip — parse(lex(source)) never panics

**Deliverable**: `schemix test.sx` prints the parsed AST. Parse errors display nicely with Ariadne.

---

### Phase 4: Abstract Syntax Tree

**Course topics**: Abstract Syntax Trees, Tree Traversal

Define the typed AST. This is the central data structure that all subsequent phases operate on.

**Concepts to revise**:

- **AST vs parse tree**: The parse tree contains every token (commas, keywords, parens). The AST abstracts these away — the tree structure _implies_ the grouping.
- **Tree traversal**: Codegen and semantic analysis are both tree traversals. In-order for codegen (emit children before parents), pre-order for semantic checks (validate parent before children).
- **Variadic functions / unions**: The course covers C unions for AST node values. In Rust, we use enums — same concept, type-safe. The `%union` in Bison maps to our `TypeExpr` enum.

**AST types**:

```rust
/// Top-level: a file contains zero or more module definitions
struct Program {
    modules: Vec<Module>,
}

/// A single module definition
struct Module {
    name: DottedName,       // e.g. "services.nginx"
    options: Vec<Option>,
    span: Span,             // For error reporting
}

/// Dotted name (stored as Vec<String> for easy decomposition)
struct DottedName {
    parts: Vec<String>,     // ["services", "nginx"]
    span: Span,
}

/// A single option within a module
struct Option {
    name: String,
    visibility: Visibility,
    type_expr: TypeExpr,
    description: Option<String>,
    default_value: Option<Literal>,
    example_value: Option<Literal>,
    span: Span,
}

/// Type expression — the core recursive type
enum TypeExpr {
    // Primitives
    Bool,
    Int,
    Str,
    Path,
    Anything,
    Package,

    // Parametric (one type parameter)
    ListOf(Box<TypeExpr>),
    AttrsOf(Box<TypeExpr>),
    NullOr(Box<TypeExpr>),
    FunctionTo(Box<TypeExpr>),

    // Composite
    Submodule(Vec<Option>),   // Recursive!
    Enum(Vec<String>),

    span: Span,
}

enum Visibility {
    None,
    Internal,
    ReadOnly,
}

enum Literal {
    Int(i64),
    Str(String),
    Bool(bool),
}
```

**Note**: The AST is defined in Phase 3's tasks but conceptually belongs here. In practice you'll define it while building the parser, since the parser needs to produce it.

---

### Phase 5: Semantic Analysis

**Course topics**: Symbol Tables, Type Checking, Error Recovery

Validate the AST before code generation.

**Checks to implement**:

1. **Duplicate option names**: No two options in the same module/submodule can share a name
2. **Empty module**: A module with no options is technically valid but deserves a warning
3. **Default type mismatch**: If a type is `int` and the default is a string, that's an error. Basic checks:
   - `bool` type → `Bool` literal
   - `int` type → `Int` literal
   - `str` type → `Str` literal
   - `path` type → `Str` literal (Nix paths are strings in option defaults)
   - `listOf` / `attrsOf` → no default check (complex defaults out of scope)
   - `enum` → default must be one of the enum values
4. **Empty enum**: `enum []` is an error
5. **Invalid dotted name**: Each part must be a valid Nix identifier
6. **Recursive submodule depth**: Warn if nesting exceeds some threshold (sanity, not a hard error)

**Concepts to revise**:

- **Symbol table**: A map from option names to their definitions. Used for duplicate detection. The course covers hash tables for symbol lookup — your `HashMap<String, Option>` serves this purpose.
- **Type checking**: The course's calculator type-checks numeric expressions. Your type-checking validates default/example values against declared types.
- **Error recovery**: Continue checking after the first error. Collect all errors, then report them all. Don't stop at the first problem.
- **Scope**: Each submodule block creates a new scope for option names.

**Tasks**:

1. Implement `semantic::validate(program: &Program) -> Vec<Diagnostic>`
2. Each diagnostic has a span (for Ariadne), a severity (error/warning), and a message
3. Wire into `main.rs`: parse → validate → on errors, print and exit
4. Unit tests for each validation rule
5. Integration tests: files that should fail validation

**Deliverable**: `schemix test.sx` catches semantic errors and reports them with Ariadne.

---

### Phase 6: Code Generation

**Course topics**: Code Generation, Tree Traversal

Walk the AST and emit Nix code.

**Concepts to revise**:

- **Tree traversal**: Depth-first, in-order traversal of the AST to emit code. Each node emits its contribution, children first.
- **Code generation strategies**: "Direct" generation (walk tree, emit strings) vs "intermediate representation" (tree → IR → code). For this project, direct generation is sufficient — the target language (Nix) is close enough to the AST.
- **Pretty printing**: Indentation, formatting. The course doesn't cover this, but it's essential for usable output.

**Emission rules**:

- Each `Module` → `options.<dotted_name> = { ... };`
- Each `Option` → `<name> = lib.mkOption { <fields> };`
- Fields emitted in order: `type`, `description`, `default`, `example`, `readOnly`, `internal`
- `type` field: recursive — `TypeExpr::Bool` → `lib.types.bool`, `TypeExpr::ListOf(inner)` → `lib.types.listOf (<inner>)`, etc.
- Submodule: `lib.types.submodule { options = { <options> }; }`
- Enum: `lib.types.enum [ <values> ]`
- Booleans in Nix: `true` / `false`
- Strings in Nix: `"..."` with escaped inner quotes
- Integers in Nix: bare numbers
- File header: `{ lib, ... }:`
- `@internal` → `internal = true;` inside mkOption
- `@readonly` → `readOnly = true;` inside mkOption

**Tasks**:

1. Implement `codegen::generate(program: &Program) -> String`
2. Handle indentation (nested submodules, module blocks)
3. Wire into `main.rs`: validate → generate → write output
4. Unit tests: known ASTs → expected Nix output strings
5. Integration tests: `.sx` files → `.nix` output → verify with `nix-instantiate --parse` or `nix eval --expr`

**Testing**:

- Unit: each AST node type emits correct Nix
- Integration: end-to-end files produce valid Nix that parses
- Property: generated Nix is syntactically valid (can be parsed by `nix-instantiate`)

**Deliverable**: `schemix test.sx -o test.nix` produces valid Nix. `nix-instantiate --parse test.nix` succeeds.

---

### Phase 7: Polish & Integration

**Tasks**:

1. Error message quality pass: re-read all Ariadne outputs, improve wording
2. Edge cases:
   - Empty input file
   - Module with zero options
   - Very deeply nested submodules
   - Unicode in strings / identifiers
   - Very long dotted names
3. Add `--version` and `--help` to CLI
4. Verify `nix build` succeeds
5. Add a `tests/` directory with:
   - `tests/lex/` — lexer test inputs
   - `tests/parse/` — parser test inputs
   - `tests/semantic/` — semantic error test inputs
   - `tests/integration/` — end-to-end `.sx` → `.nix` pairs
6. Consider adding a `schemix eval` subcommand that reads Nix to validate output (stretch goal)

---

## 3. Testing Strategy

Three levels:

### Unit Tests

- Inline `#[cfg(test)]` in each module
- Test individual functions: tokenization, parsing productions, validation rules, codegen snippets
- Use `pretty_assertions` crate for readable diff output on failures

### Property Tests

- Lexer: arbitrary valid text → only valid tokens
- Parser: arbitrary valid token sequences → valid AST or clean error
- Round-trip: generate Nix → parse with nix-instantiate → verify valid

### Integration Tests

- In `tests/` directory
- Each test: read `.sx` file → compile → write `.nix` → validate with `nix-instantiate --parse`
- Error tests: compile invalid `.sx` → verify expected error message

---

## 4. Key Files

| File              | Purpose                                              |
| ----------------- | ---------------------------------------------------- |
| `Cargo.toml`      | Dependencies: chumsky, ariadne, clap                 |
| `src/main.rs`     | CLI entry, pipeline orchestration                    |
| `src/lexer.rs`    | Token enum + Chumsky lexer                           |
| `src/ast.rs`      | AST types                                            |
| `src/parser.rs`   | Chumsky parser                                       |
| `src/semantic.rs` | Validation pass                                      |
| `src/codegen.rs`  | Nix code emitter                                     |
| `src/error.rs`    | Error types + Ariadne formatting                     |
| `src/span.rs`     | Span types for Ariadne                               |
| `nix/package.nix` | Nix build definition (already exists, minor updates) |
| `tests/`          | Integration test files                               |

---

## 5. Course Topic → Implementation Mapping

| Course Topic                     | Where You Apply It                                            |
| -------------------------------- | ------------------------------------------------------------- |
| Introduction / Compiler Pipeline | Phase 1: Overall pipeline design                              |
| Lexical Analysis                 | Phase 2: Chumsky lexer                                        |
| Regular Expressions              | Phase 2: Token patterns                                       |
| Regular Languages / NFA / DFA    | Phase 2: Understanding how Chumsky compiles patterns          |
| Thompson's Construction          | Phase 2: Regex → NFA conversion (understand, don't implement) |
| Subset Construction              | Phase 2: NFA → DFA conversion (understand, don't implement)   |
| Calculator (Flex/Bison)          | Phase 2-3: Your lexer+parser replaces Flex+Bison              |
| Context-Free Grammars            | Phase 3: Grammar design                                       |
| BNF                              | Phase 3: Grammar notation                                     |
| Parse Trees vs ASTs              | Phase 4: AST design                                           |
| Associativity                    | Phase 3: Type constructor associativity                       |
| Precedence                       | Phase 3: Grammar structure encodes precedence                 |
| Top-Down / Recursive Descent     | Phase 3: Chumsky generates recursive-descent                  |
| Predictive / LL(1)               | Phase 3: Understand lookahead requirements                    |
| Bottom-Up / Shift-Reduce         | Phase 3: Compare with Bison's approach                        |
| SR Conflicts                     | Phase 3: Understand why your grammar is unambiguous           |
| Symbol Tables                    | Phase 5: Duplicate detection uses a HashMap                   |
| Code Generation                  | Phase 6: AST → Nix string emission                            |
| Advanced Calculator              | Phase 6: Submodules parallel user-defined functions           |

---

## 6. Dependencies

```toml
[dependencies]
chumsky = "0.10"     # Parser combinator
ariadne = "0.5"      # Span-based error reporting
clap = { version = "4", features = ["derive"] }  # CLI argument parsing

[dev-dependencies]
pretty_assertions = "1"  # Readable test diffs
```
