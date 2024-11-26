# Vaca2

Vaca2 is a programming language, but why it's special?

Vaca2 is a LISP language that uses [edn](https://github.com/edn-format/edn)-like syntax and compiles to C

## Roadmap

Vaca2 is still in planning phase but the roadmap is as follows:

- [ ] Syntax definition
- [ ] Parser implementation
- [ ] Code generation
    - [ ] Assignments
    - [ ] Call to native functions
    - [ ] Type validation
    - [ ] Function declaration
    - [ ] Macro declaration
    - [ ] Type declaration
- [ ] Project management tooling
- [ ] LSP tooling

## What Is Vaca

Vaca main target is to be Clojure equivalent for C. Clojure is a LISP that runs on the JVM. Vaca is meant to be a LISP that compiles to C with full interop with C code.

Vaca is meant to be a multi paradigm garbage collected programming language

## Tech Stack

- GC implementation: https://github.com/mkirchner/gc
