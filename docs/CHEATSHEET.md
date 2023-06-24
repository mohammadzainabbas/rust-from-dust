# Rust Cheatsheet

## Table of Contents
1. [Basic Types & Variables](#basic-types-&-variables)
1. [Control Flow](#control-flow)
1. [References, Ownership, and Borrowing](#references-ownership-and-borrowing)
1. [Pattern Matching](#pattern-matching)
1. [Iterators](#iterators)
1. [Error Handling](#error-handling)
    * [Combinators](#combinators)
    * [Multiple error types](#multiple-error-types)
    * [Iterating over errors](#iterating-over-errors)
1. [Generics, Traits, and Lifetimes](#generics-traits-and-lifetimes)
1. [Functions, Function Pointers & Closures](#functions-function-pointers-&-closures)
1. [Pointers](#pointers)
1. [Smart pointers](#smart-pointers)
1. [Packages, Crates, and Modules](#packages-crates-and-modules)

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
