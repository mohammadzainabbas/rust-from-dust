# Rust - Cheatsheet :books:

## Table of Contents

1. [Basic Types & Variables :pencil2:](#basic-types--variables-pencil2)
2. [Control Flow :arrows_clockwise:](#control-flow-arrows_clockwise)
3. [References, Ownership, and Borrowing :link:](#references-ownership-and-borrowing-link)
4. [Pattern Matching :recycle:](#pattern-matching-recycle)
5. [Iterators :repeat:](#iterators-repeat)
6. [Error Handling :exclamation:](#error-handling-exclamation)
   - [Combinators :link:](#combinators-link)
   - [Multiple error types :warning:](#multiple-error-types-warning)
   - [Iterating over errors :arrows_counterclockwise:](#iterating-over-errors-arrows_counterclockwise)
7. [Generics, Traits, and Lifetimes :hourglass_flowing_sand:](#generics-traits-and-lifetimes-hourglass_flowing_sand)
8. [Functions, Function Pointers & Closures :performing_arts:](#functions-function-pointers--closures-performing_arts)
9. [Pointers :arrow_right:](#pointers-arrow_right)
   - [Smart pointers :brain:](#smart-pointers-brain)
10. [Packages, Crates, and Modules :package:](#packages-crates-and-modules-package)

#

## Basic Types & Variables :pencil2:

In Rust, data types are explicitly declared. This table provides an overview of basic types and variables:

| Type | In Rust |
| ---- | ------- |
| Boolean | `bool` |
| Unsigned integers | `u8, u16, u32, u64, u128` |
| Signed integers | `i8, i16, i32, i64, i128` |
| Floating point numbers | `f32, f64` |
| Platform specific integers | `usize, isize` |
| Unicode scalar value | `char` |
| String slice | `&str` |
| Owned string | `String` |

> [!NOTE]
> `usize` and `isize` are pointer-sized integers (i.e: they have the same number of bits as the platform's pointer type). For e.g: on 64-bit systems, they are 64-bit wide and On 32-bit systems, they are 32-bit wide.

### Tuple :two_men_holding_hands:
```rust
let coordinates = (82, 64);
let score = ("Team A", 12);
```

### Array & Slice :pizza:
```rust
// Arrays must have a known length and all elements must be initialized
let array = [1, 2, 3, 4, 5];
let array2 = [0; 3]; // [0, 0, 0]

// Unlike arrays the length of a slice is determined at runtime
let slice = &array[1 .. 3];
```

### HashMap :file_folder:
```rust
use std::collections::HashMap;

let mut subs = HashMap::new();
subs.insert(String::from("LGR"), 100000);
// Insert key if it doesn't have a value
subs.entry("Golang Dojo".to_owned()).or_insert(3);
```

### Struct :house:
```rust
// Definition
struct User {
  username: String,
  active: bool,
}

// Instantiation
let user = User {
  username: String::from("Mohammad Zain Abbas"),
  active: true,
};

// Tuple struct
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

### Enum :flags:
```rust
// Definition
enum Command {
  Quit,
  Move { x: i32, y: i32 },
  Speak(String),
  ChangeBGColor(i32, i32, i32),
}

// Instantiation
let msg1 = Command::Quit;
let msg2 = Command::Move{ x: 1, y: 2 };
let msg3 = Command::Speak("Hi".to_owned());
let msg4 = Command::ChangeBGColor(0, 0, 0);
```

### Constant :lock:
```rust
const MAX_POINTS: u32 = 100_000;
```

### Static variable :globe_with_meridians:
```rust
// Unlike constants static variables are
// stored in a dedicated memory location
// and can be mutated.
static MAJOR_VERSION: u32 = 1;
static mut COUNTER: u32 = 0;
```

### Mutability :wrench:
```rust
let mut x = 5;
x = 6;
```

### Shadowing :last_quarter_moon:
```rust
let x = 5;
let x = x * 2;
```

### Type alias :label:
```rust
// `NanoSecond` is a new name for `u64`.
type NanoSecond = u64;
```

#

## Control Flow :arrows_clockwise:

### if and if let :question:
```rust
let num = Some(22);

if num.is_some() {
  println!("number is: {}", num.unwrap());
}

// match pattern and assign variable
if let Some(i) = num {
  println!("number is: {}", i);
}
```

### loop :arrows_counterclockwise:
```rust
let mut count = 0;
loop {
  count += 1;
  if count == 5 {
    break; // Exit loop
  }
}
```

### Nested loops & labels :arrows_clockwise:
```rust
'outer: loop {
  'inner: loop {
    // This breaks the inner loop
    break;
    // This breaks the outer loop
    break 'outer;
  }
}
```

### Returning from loops :fast_forward:
```rust
let mut counter = 0;

let result = loop {
  counter += 1;

  if counter == 10 {
    break counter;
  }
};
```

### while and while let :hourglass_flowing_sand:
```rust
while n < 101 {
  n += 1;
}

let mut optional = Some(0);

while let Some(i) = optional {
  print!("{}", i);
}
```

### for loop :loop:
```rust
for n in 1..101 {
  println!("{}", n);
}

let names = vec!["Mohammad", "Zain", "Abbas"];

for name in names.iter() {
  println!("{}", name);
}
```

### match :recycle:
```rust
let optional = Some(0);

match optional {
  Some(i) => println!("{}", i),
  None => println!("No value.")
}

let msg = Command::Move { x: 1, y: 1 };

match msg {
  Command::Quit => println!("Quit"),
  Command::Move { x, y } => println!("Move to {}, {}", x, y),
  Command::Speak(txt) => println!("Speak: {}", txt),
  Command::ChangeBGColor(r, g, b) => println!("Change BG Color to {}, {}, {}", r, g, b),
}
```

### match with if :recycle: :question:
```rust
let pair = (2, -2);

match pair {
  (x, y) if x == y => println!("These are twins"),
  (x, y) if x + y == 0 => println!("These are opposites"),
  _ => (),
}
```

#

## References, Ownership, and Borrowing :link:

### Ownership :gift:
```rust
// s1 owns the memory for "hello"
let s1 = String::from("hello");

// ownership of the memory is moved to s2
let s2 = s1;

// This line would throw an error because s1 no longer has ownership
// println!("{}, world!", s1);
```

### Borrowing :bank:
```rust
// s1 owns the memory for "hello"
let s1 = String::from("hello");

// s2 borrows a reference to the memory
let s2 = &s1;

// Both s1 and s2 are valid here
println!("{}, world!", s1);
println!("{}, world!", s2);
```

### Mutable borrowing :bank: :wrench:
```rust
// s1 owns the memory for "hello"
let mut s1 = String::from("hello");

// s2 borrows a mutable reference to the memory
let s2 = &mut s1;

// s2 can mutate the memory
s2.push_str(", world!");

// println!("{}, world!", s1); would throw an error because s1 is not allowed to be used while it is mutably borrowed
println!("{}", s2);
```

### References in Structs :link: :house:
```rust
struct User<'a> {
    name: &'a str,
    age: u8,
}

let user = User {
    name: "Alice",
    age: 30,
};
```

## Functions :gear:

### Definition :construction_worker:
```rust
fn add_two(x: i32) -> i32 {
    x + 2
}
```

### Parameters :point_left:
```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

### Return Values :point_right:
```rust
fn five() -> i32 {
    5
}
```

### Functions in Structs :gear: :house:
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

## 📚 References, Ownership, and Borrowing

📝 **Ownership rules**
In Rust, each value has a variable that is considered its owner. The important rules are:

- 🏷️ Each value in Rust has a variable that’s called its owner.
- 🚫 There can only be one owner at a time.
- ❌ When the owner goes out of scope, the value will be dropped.

📝 **Borrowing rules**
Rust allows borrowing of values under certain rules:

- 🔄 At any given time, you can have either one mutable reference or any number of immutable references.
- ✔️ References must always be valid.

💡 Here's how to create references in Rust:

```rust
let s1 = String::from("hello world!");
let s1_ref = &s1; // immutable reference

let mut s2 = String::from("hello");
let s2_ref = &mut s2; // mutable reference

s2_ref.push_str(" world!");
```

## 🔄 Copy, Move, and Clone

In Rust, handling of variables depends on whether they implement the Copy trait, and how the values are assigned:

```rust
// Simple values which implement the Copy trait are copied by value
let x = 5;
let y = x;

println!("{}", x); // x is still valid

// The string is moved to s2 and s1 is invalidated 
let s1 = String::from("Let's Get Rusty!");
let s2 = s1; // Shallow copy a.k.a move

println!("{}", s1); // Error: s1 is invalid

let s1 = String::from("Let's Get Rusty!");
let s2 = s1.clone(); // Deep copy

// Valid because s1 isn't moved
println!("{}", s1);
```

## 📋 Ownership and Functions

In Rust, the ownership rules apply to function arguments and return values. Here's an example:

```rust
fn main() {
  let x = 5;
  takes_copy(x); // x is copied by value

  let s = String::from("Let’s Get Rusty!");
  // s is moved into the function
  takes_ownership(s);
  
  // return value is moved into s1
  let s1 = gives_ownership();
  
  let s2 = String::from("LGR");
  let s3 = takes_and_gives_back(s2);
}

fn takes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
} // some_string goes out of scope and drop is called. The backing memory is freed.

fn gives_ownership() -> String {
  let some_string = String::from("LGR");
  some_string
}

fn takes_and_gives_back(some_string: String) -> String {
  some_string
}
```
