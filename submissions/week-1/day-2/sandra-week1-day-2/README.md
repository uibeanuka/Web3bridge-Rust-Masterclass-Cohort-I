### Task 1: Study Chapters of The Rust Book,Study the **remaining of Chapter 2** and **all of Chapter 3** of [The Rust Programming Language](https://doc.rust-lang.org/book/)

#### Solution

##### Chapter 2

I was able to understand the following from the remaining part of chapter 2
✅ Bringing in dependencies: std::io, rand, and std::cmp::Ordering,  
✅ Reading user input with io::stdin().read_line(&mut guess) and handling potential errors using .expect()
✅ Converting the input string to a number using .trim().parse(), matching on Ok(num) or Err(\_)
✅ Using match guess.cmp(&secret_number) to compare guesses and give feedback (Less, Greater, Equal)
✅ I was also able to understand methods, and external crates at a high level
✅ in Summmary I able to understand basic I/O, error handling (Result), conditionals (match), loops, and modular project organization.

##### Chapter 3

###### Function Declaration & Execution in Rust

✅ Defined using the `fn` keyword  
✅ Functions in Rust follows **snake_case** naming convention.
✅ How Function Parameters must have explicitly declared types.

###### Control flow

✅ how to use if expressions to decide which block of code to run based on a Boolean condition.
✅ how to use the three main Rust loop types .while (conditional), and for (iterates over collections or ranges).
✅ How Function Parameters must have explicitly declared types.
✅ How and when to use the for loop

#### Task 2: Basic Arithmetic in Rust

#### Solutions

##### `add(a: f32, b: f32) -> f32`

Performs addition of two floating-point numbers.

##### `sub(a: f32, b: f32) -> f32`

Performs subtraction of two floating-point numbers.

##### `mul(a: f32, b: f32) -> f32`

Performs multiplication of two floating-point numbers.

##### `div(a: f32, b: f32) -> f32`

##### Screenshot of Program Output

![Rust Calculator with Colored Output](screenshot.png)
