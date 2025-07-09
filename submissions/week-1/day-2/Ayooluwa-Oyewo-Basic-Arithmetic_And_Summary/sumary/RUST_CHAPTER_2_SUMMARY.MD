# Variables and Mutability in Rust

## 🟩 Immutability by Default

In Rust, variables are immutable by default – once a value is assigned, it cannot be changed.

✅ This is a safety feature that helps prevent bugs and makes code easier to reason about, especially in concurrent environments.

```rust
let x = 5;
x = 6; // ❌ Compile-time error: cannot assign to immutable variable
```

## 🟦 Making Variables Mutable

You can make a variable mutable using `mut`:

```rust
let mut x = 5;
x = 6; // ✅ This works
```

✍️ Mutability is explicit, signaling to readers that the value will change.

## 🟨 Constants

Constants are declared using `const` (not `let`) and are always immutable.

### Rules:
- Must include a type annotation.
- Must be set to a constant expression (not a runtime value).
- Can be declared in any scope, including global.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

🧠 Naming convention: Use `ALL_CAPS_WITH_UNDERSCORES`.

## 🟧 Shadowing

Rust allows you to reuse a variable name using `let` to create a new variable in the same or inner scope.

```rust
let x = 5;
let x = x + 1;       // x is now 6
{
    let x = x * 2;   // x is 12 only in this inner scope
}
println!("{x}");     // prints 6
```

### 🔍 Shadowing vs `mut`

- Shadowing creates a new variable (even with the same name).
- It allows changing types safely.
- It avoids accidental reassignment errors.

```rust
// Shadowing (this works):
let spaces = "   ";
let spaces = spaces.len(); // ✅ changes from &str to usize

// Mutability with type change (this fails):
let mut spaces = "   ";
spaces = spaces.len(); // ❌ Compile-time type error
```

## ✅ Conclusion

Rust’s variable handling promotes safe and maintainable code.

- Use `mut` when a value needs to change.
- Use `const` for global, fixed values.
- Use shadowing to:
  - Transform or reuse variables.
  - Change types while keeping immutability benefits.


# 🦀 Data Types, Functions, Comments & Control Flow

Rust is a systems programming language focused on safety, speed, and concurrency. In this article, we'll explore key foundational topics: **data types**, **functions**, **comments**, and **control flow**. These concepts form the bedrock of writing expressive and safe Rust code.

---

## 🧠 Data Types in Rust

Rust is a **statically typed** language, meaning all variable types must be known at compile time. While the compiler can often **infer** types, **explicit annotations** are sometimes necessary:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

### 🔢 Scalar Types

Scalar types represent **single values** and include:

* **Integers** (`i32`, `u8`, etc.)
* **Floating-point numbers** (`f32`, `f64`)
* **Booleans** (`bool`)
* **Characters** (`char`)

#### Integer Types

Rust offers signed and unsigned integers in multiple sizes:

| Size | Signed  | Unsigned |
| ---- | ------- | -------- |
| 8    | `i8`    | `u8`     |
| 16   | `i16`   | `u16`    |
| 32   | `i32`   | `u32`    |
| 64   | `i64`   | `u64`    |
| 128  | `i128`  | `u128`   |
| arch | `isize` | `usize`  |

**Defaults:** `i32` for integers, `f64` for floats.

#### Overflow Behavior

* **Debug mode**: panics on overflow
* **Release mode**: wraps (two's complement)
* Use methods like `wrapping_add`, `checked_add` to handle overflow explicitly

#### Floating-Point, Booleans, and Characters

```rust
let x = 2.0;       // f64
let y: f32 = 3.0;  // f32
let is_ready = true;
let heart = '❤️';
```

---

### 🧱 Compound Types

#### Tuples

Tuples can hold values of different types:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
println!("{}", tup.0); // Access by index
```

#### Arrays

Arrays hold multiple values of the same type and fixed length:

```rust
let a = [1, 2, 3, 4, 5];
let repeated = [3; 5]; // [3, 3, 3, 3, 3]
```

> ⚠️ Accessing an out-of-bounds index will panic at runtime.

---

## 🛠️ Functions in Rust

Functions are defined using the `fn` keyword. The entry point of every Rust program is the `main` function:

```rust
fn main() {
    println!("Hello, world!");
}
```

### 📥 Parameters & Type Annotations

Function parameters must be explicitly typed:

```rust
fn print_label(value: i32, unit: char) {
    println!("{value}{unit}");
}
```

### 📐 Statements vs Expressions

* **Statements** do things but return nothing
* **Expressions** evaluate to a value

```rust
let y = {
    let x = 3;
    x + 1  // no semicolon means it's returned
};
```

> ❗ Adding a semicolon turns an expression into a statement, which returns `()`.

### 🔁 Return Values

Functions return the last expression implicitly:

```rust
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Use `return` if you want to return early:

```rust
fn early_return(x: i32) -> i32 {
    if x < 0 {
        return 0;
    }
    x
}
```

---

## 💬 Comments in Rust

Rust uses `//` for single-line comments:

```rust
// This is a comment
let x = 5; // Inline comment
```

For multi-line explanations:

```rust
// Explain something
// over several lines.
```

> 💡 Comments enhance readability and do not affect performance or compilation.

---

## 🔀 Control Flow in Rust

Rust supports conditional execution and repetition through:

### ✅ if Expressions

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("Less than 5");
    } else if number > 5 {
        println!("Greater than 5");
    } else {
        println!("Exactly 5");
    }
}
```

* Conditions **must** be booleans.
* You can use `if` as an expression:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

> ⚠️ Both `if` and `else` arms must return the same type.

---

### 🔁 Loops

Rust has three looping constructs: `loop`, `while`, and `for`.

#### 🔂 Infinite Loop with `loop`

```rust
loop {
    println!("again!");
}
```

Use `break` to exit and `continue` to skip iterations. Loops can also return values:

```rust
let result = loop {
    if some_condition {
        break 42;
    }
};
```

#### 🏷️ Loop Labels

Used when breaking out of nested loops:

```rust
'outer: loop {
    loop {
        break 'outer;
    }
}
```

#### 🔁 Conditional Loops with `while`

```rust
let mut x = 3;
while x > 0 {
    println!("{x}");
    x -= 1;
}
println!("LIFTOFF!");
```

#### 🔁 Iterating Collections with `for`

```rust
let a = [10, 20, 30];
for element in a {
    println!("{element}");
}
```

Even countdowns can be done elegantly:

```rust
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!!!");
```

> ✅ `for` loops are preferred for safe and idiomatic iteration.

---

## 🧩 Conclusion

* **Data types** ensure safety at compile time
* **Functions** are expressive with strong typing and implicit return
* **Comments** aid clarity
* **Control flow** via `if`, `loop`, `while`, and `for` lets you express logic clearly and safely

Mastering these building blocks sets the foundation for writing clean, safe, and performant Rust code. 🚀
