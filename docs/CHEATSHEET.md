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
subs.entry("Golang Dojo".to_owned()) .or_insert(3);
```

### Struct :house:
```rust
// Definition
struct User {
  username: String,
  active: bool,
}

// Instantiation
let user1 = User {
  username: String::from("bogdan"),
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

### while :hourglass_flowing_sand:
```rust
let mut counter = 0;

while counter != 10 {
  counter += 1;
}
```

### for :loop:
```rust
for number in (1..4).rev() {
  println!("{}!", number);
}
```

### match :recycle:
```rust
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
