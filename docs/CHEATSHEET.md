# Rust Cheatsheet

## Table of Contents
1. [Basic Types & Variables](#basic-types-&-variables)
2. [Control Flow](#control-flow)
3. [References, Ownership, and Borrowing](#references-ownership-and-borrowing)
4. [Pattern Matching](#pattern-matching)
5. [Iterators](#iterators)
6. [Error Handling](#error-handling)
7. [Combinators](#combinators)
8. [Multiple error types](#multiple-error-types)
9. [Iterating over errors](#iterating-over-errors)
10. [Generics, Traits, and Lifetimes](#generics-traits-and-lifetimes)
11. [Functions, Function Pointers & Closures](#functions-function-pointers-&-closures)
12. [Pointers](#pointers)
13. [Smart pointers](#smart-pointers)
14. [Packages, Crates, and Modules](#packages-crates-and-modules)

YouTube Channel: [Let's Get Rusty](https://www.youtube.com/c/LetsGetRusty)

## Basic Types & Variables
`bool` - Boolean

Unsigned integers
`u8`, `u16`, `u32`, `u64`, `u128`

Signed integers
`i8`, `i16`, `i32`, `i64`, `i128`

Floating point numbers
`f32`, `f64`

Platform specific integers
`usize` - Unsigned integer. Same number of bits as the platform's pointer type.

`isize` - Signed integer. Same number of bits as the platform's pointer type.

`char` - Unicode scalar value
`&str` - String slice
`String` - Owned string

**Tuple**
```rust
let coordinates = (82, 64);
let score = ("Team A", 12);
