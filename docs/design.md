# Design

## Values

Simple
Small
Expressive

## Goals

These may or may not already be in the language.

* Simple, preferably word-based syntax.
* Batteries-included core library.
* Immutability by default.
* Expressiveness:
    * Pattern matching/destructuring
    * Lambdas
* Acceptable performance.
* Fast (sub-second for smaller projects) build times.
* Small binaries.
* Easy foreign function interfaces (FFI) - enabling library code in faster languages, especially C.
* WebAssembly as a tier 1 compilation target.

## Obstacles

These are non-goals - things that will not be added to the language.

* Null - use option/either/result instead.
* Manual memory management.
* Ownership/lifetime rules and references.
* Lifetimes.
* Separate syntax for generics.
    * Function parameters and record fields are instead expressed as either a single concrete type or possibly one or more traits.  
