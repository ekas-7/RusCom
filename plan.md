# Plan: Build a C++ Compiler in Rust

This document outlines a practical, phased plan to build a C++ compiler front-to-back using Rust. It focuses on building a working, maintainable compiler for a useful subset of C++ first, then expanding toward full language coverage. Each phase lists concrete tasks, success criteria, and verification steps.

## Goal / Scope

- Primary goal: implement a modern, maintainable C++ compiler written in Rust that can compile a well-defined subset of C++ (later expanded toward full C++).
- Target output: native object files and executables (ELF/macOS Mach-O/PE) using system linkers, or LLVM IR as an intermediate target.
- Development strategy: incremental — minimal working compiler for a subset, then add features.

## High-level contract (inputs/outputs)

- Inputs: C++ source files (.cpp/.cc/.cxx, .h/.hpp headers as needed), build flags (include paths, macros, optimization level), target triple.
- Outputs: object files (.o/.obj), assembly, LLVM IR (optional), or final linked executable.
- Errors: clear diagnostics for syntax, semantic, and linking errors; graceful failure modes for unimplemented features.

## Assumptions

- We'll implement a pragmatic subset of C++ first: functions, classes (single inheritance), basic templates (type alias templates), fundamental types, references/pointers, structs, constructors/destructors, simple overload resolution, simple templates, and no RTTI/complex vtables initially.
- Use LLVM for codegen (via `inkwell` or `llvm-sys`) to avoid reimplementing code generation, object file emission, and optimizations.
- Use Rust toolchain for development and cargo for packaging.

## Milestones

1. Project setup & tooling
2. Lexer + parser for a small C++ subset
3. AST + pretty printing
4. Name resolution and semantic checks
5. Type system and basic type checking
6. IR lowering (to LLVM IR) and codegen
7. Linker integration and runtime support
8. Optimizations and additional language features
9. Testing, CI, fuzzing
10. Packaging & docs

---

## Phase 1 — Project setup & tooling

Tasks:
- Create a Rust workspace using Cargo.
- Add dependencies: `inkwell` or `llvm-sys` (choose `inkwell` for easier API), `nom` or `pest` for parsing, `thiserror`/`anyhow` for error handling, `log`/`env_logger` for diagnostics.
- Add a basic CLI (using `clap`) with subcommands: `compile`, `assemble`, `emit-llvm`, `run`.
- Create a minimal README and contributing notes.

Success criteria:
- `cargo build` succeeds, `cargo run -- --help` prints usage.

## Phase 2 — Lexer & Parser

Tasks:
- Decide parser strategy: hand-written recursive-descent for a controlled subset, or generate parser with `pest` for grammar-driven approach. Recommendation: start with `nom`/hand-written parser for clearer error control.
- Implement a Unicode-aware lexer that produces tokens with source locations.
- Implement a parser for translation units: declarations, function definitions, simple class/struct declarations, variable declarations, expressions, control flow (if/while/for), return statements, and simple templates.
- Build AST node types with source spans.

Success criteria:
- Parser accepts simple programs and rejects invalid syntax with useful error spans.

Edge cases:
- Preprocessor: for initial subset, support a minimal preprocessor or require preprocessed input via `cpp`/`clang -E` externally. Later phases will add a preprocessor.
- Templates and macros — postpone full support.

## Phase 3 — AST and pretty printing

Tasks:
- Define AST types (enums/structs) to represent declarations, statements, expressions, types, and templates.
- Implement an AST to-string printer for debugging (with spans).
- Add a simple REPL or `ast-dump` CLI command for printing parsed AST.

Success criteria:
- CLI `ast-dump` prints readable AST for test inputs.

## Phase 4 — Name resolution and semantic analysis

Tasks:
- Implement symbol tables, scopes (global, namespace, class, function), and lookup rules.
- Handle overloading resolution for functions (basic rules), name hiding, and scoping rules.
- Implement basic diagnostics and recovery strategies.

Success criteria:
- Name resolution passes for non-ambiguous programs and reports meaningful errors for unresolved references.

## Phase 5 — Type system and type checking

Tasks:
- Implement a type representation (builtins, pointers, references, arrays, function types, class types, template type parameters).
- Implement type inference for auto and basic conversions as needed.
- Implement overload resolution, implicit conversions, and basic template instantiation (deferred until later if too complex).
- Implement constructors/destructors semantics for local variables.

Success criteria:
- Typechecker accepts simple programs and rejects type errors with helpful diagnostics.

Edge cases:
- Template-heavy code, SFINAE, and advanced overload resolution are deferred.

## Phase 6 — IR lowering and code generation

Tasks:
- Choose backend: LLVM via `inkwell` (recommended) or Cranelift for faster compilation. LLVM provides mature optimizations and object file emission.
- Map AST typed constructs to LLVM IR, handle calling conventions and name mangling (Itanium C++ ABI option).
- Implement vtables for virtual functions, memory layout for classes (single inheritance initially), and ABI-compliant function signatures.
- Support emitting object files and assembly and linking with system toolchain.

Success criteria:
- Compiler can generate an executable for simple programs (hello world, basic class methods) producing correct output.

## Phase 7 — Linking & runtime

Tasks:
- Integrate with system linker: produce object files and invoke `ld/clang` to link or use LLVM's LLD.
- Provide a minimal runtime: constructors for static objects, basic C++-like exception stubs (if implementing exceptions), and necessary CRT glue.

Success criteria:
- Successfully compile and link small programs into executables.

## Phase 8 — Optimizations & additional features

Tasks:
- Implement standard optimizations (inlining, dead-code elimination, simple loop optimizations) by leveraging LLVM passes.
- Add more C++ features: templates, multiple inheritance, move semantics, rvalue references, RTTI, exceptions.

Success criteria:
- Performance comparable to -O0 for correctness; optimizations improve performance for typical code.

## Phase 9 — Testing, fuzzing, CI

Tasks:
- Add unit tests for lexer, parser, semantic analyzer, and codegen. Use `cargo test`.
- Add integration tests: compile small programs and check output and/or return codes.
- Add fuzzing harness (cargo-fuzz) to fuzz parser and typechecker.
- Set up CI (GitHub Actions) to run tests and build artifacts.

Success criteria:
- CI runs on each PR and verifies build/tests.

## Phase 10 — Packaging & docs

Tasks:
- Create a CLI with clear flags, man pages, and package binaries for macOS, Linux, and Windows (GitHub Releases).
- Document supported C++ subset, developer guide, and contribution process.

Success criteria:
- Users can download releases and compile simple programs following the README.

---

## Tools, libraries & notes

- Rust ecosystem: `cargo`, `clap`, `thiserror`, `anyhow`, `log`, `env_logger`.
- Parsing: `nom`, `pest`, or hand-written recursive-descent.
- LLVM bindings: `inkwell` (high-level) or `llvm-sys` (lower-level). `inkwell` recommended.
- For linking: `lld` or system `clang`/`ld`.
- Optional JIT/prototyping: `cranelift`/`wasmtime` for experimentation.

## Risks and mitigation

- C++ is huge: reduce scope initially and expand. Implement a well-documented subset.
- ABI correctness: adopt Itanium C++ ABI naming and test against Clang/GCC outputs.
- Templates and ODR: implement conservative checks early and iterate.
- Long-term maintenance: keep components modular.

## Quick project layout (initial)

- /src
  - main.rs (CLI)
  - lexer.rs
  - parser.rs
  - ast.rs
  - sema.rs
  - types.rs
  - codegen/
    - mod.rs
    - llvm_backend.rs
  - util.rs
- Cargo.toml
- README.md
- tests/

## Acceptance tests (examples)
- Hello world
- Class with method calls
- Simple template instantiation

## Next steps (immediate)
1. Scaffold Cargo project and add dependencies.
2. Implement a lexer for the C++ subset.
3. Implement a simple parser to parse functions and print AST.

---

If you'd like, I can add this `plan.md` to the repository now (I have created it). I can also scaffold a Rust project with Cargo, add a minimal lexer/parser starter, or turn the plan into a checklist of issues. Tell me which next action you'd like.