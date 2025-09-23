# Checklist: Build a C++ Compiler in Rust

This checklist converts `plan.md` into explicit, actionable items you can tick off as you implement the compiler. Work top-to-bottom; break large items into smaller tasks as needed.

## Phase 0 — Prep
- [x] Create `plan.md` documenting overall strategy and milestones.
- [ ] Create this `checklist.md` (you are here).

## Phase 1 — Project setup & tooling
- [ ] Initialize Rust workspace (Cargo) and repository layout.
- [ ] Add dependencies: `clap`, `inkwell` (or `llvm-sys`), `nom` or `pest`, `thiserror`, `anyhow`, `log`, `env_logger`.
- [ ] Create `src/main.rs` with CLI and subcommands: `compile`, `assemble`, `emit-llvm`, `run`.
- [ ] Add `README.md` and contribution notes.
- [ ] Verify `cargo build` and `cargo run -- --help`.

## Phase 2 — Lexer & Parser
- [ ] Choose parser approach: hand-written recursive-descent (`nom`) or grammar-based (`pest`).
- [ ] Implement lexer producing tokens with source spans.
- [ ] Implement parser for translation units: declarations, function defs, classes/structs, var decls, expressions, control flow, return.
- [ ] Support minimal preprocessing strategy (initially: require preprocessed input or integrate with `cpp`).
- [ ] Add tests for lexer and parser; `cargo test` passes for these units.

## Phase 3 — AST & pretty printing
- [ ] Define AST node types (declarations, statements, expressions, types, templates) with spans.
- [ ] Implement AST pretty-printer / debug dumper.
- [ ] Add `ast-dump` CLI subcommand.
- [ ] Add tests that parse code and assert AST shape or pretty output.

## Phase 4 — Name resolution & semantics
- [ ] Implement symbol tables and scoped lookups (global, namespaces, classes, functions).
- [ ] Implement overload resolution basics.
- [ ] Implement name hiding rules and diagnostics for ambiguous or missing names.
- [ ] Add tests for scoping and overload resolution.

## Phase 5 — Type system & type checking
- [ ] Implement type representation: builtins, pointers, refs, arrays, func types, class types.
- [ ] Implement constructors/destructors for local variables.
- [ ] Implement basic conversions and overload resolution integration.
- [ ] Implement simple template support (type alias templates or basic instantiation) or document as deferred.
- [ ] Add unit tests for type checking.

## Phase 6 — IR design & lowering
- [ ] Choose IR strategy: lower to LLVM IR via `inkwell` or design a custom IR (SSA).
- [ ] Map language types to IR types and calling conventions.
- [ ] Implement lowering passes for functions, control flow, and expressions.
- [ ] Add tests that lower AST to IR and validate against expected IR patterns.

## Phase 7 — Codegen & backend
- [ ] Implement code generation pipeline to emit LLVM IR and object files.
- [ ] Implement name mangling following Itanium C++ ABI (or a simplified scheme initially).
- [ ] Implement vtables and simple class layout for single inheritance.
- [ ] Integrate with `lld` or system linker to produce executables.
- [ ] Add end-to-end integration tests (compile small programs and run them).

## Phase 8 — Linking & runtime
- [ ] Implement runtime stubs for constructors of static objects and necessary CRT glue.
- [ ] Support linking with system libraries and standard C++ libraries if needed.
- [ ] Verify linking on macOS (Mach-O) and Linux (ELF) as available.

## Phase 9 — Optimizations & additional features
- [ ] Plug into LLVM optimization passes and expose `-O` flags.
- [ ] Implement selective optimizations: inlining, DCE, simple loop optimizations.
- [ ] Add more C++ features incrementally: templates, exceptions, move semantics, RTTI, multiple inheritance.
- [ ] Add tests for each new feature and maintain regression suite.

## Phase 10 — Testing, fuzzing, CI
- [ ] Add unit tests (lexer/parser/sema/codegen) and `cargo test` coverage.
- [ ] Add integration tests (compile+run small programs) using temporary directories.
- [ ] Add fuzzing for parser/typechecker (`cargo-fuzz`).
- [ ] Create GitHub Actions: build, test, and run integration tests on PRs.

## Phase 11 — Packaging & distribution
- [ ] Build CLI with user-friendly flags and help text.
- [ ] Create packaging scripts for macOS/Linux/Windows (tar.gz, .zip, or GitHub Releases).
- [ ] Provide installation instructions and sample usage in `README.md`.

## Phase 12 — Documentation & examples
- [ ] Document supported C++ subset and roadmap.
- [ ] Add example programs (hello world, class methods, template examples).
- [ ] Add contributor guide and coding standards.

---

Notes:
- Break large items (like parser or templates) into smaller issues when implementing.
- Prefer conservative, test-driven steps: implement, test, then expand.

If you want, I can now:
- Commit `checklist.md` to the repository (already created) and mark the todo as completed, or
- Scaffold the Cargo project and implement the initial CLI.

Tell me which next action you'd like.