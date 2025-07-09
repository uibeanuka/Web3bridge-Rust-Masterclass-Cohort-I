
### ✏️ Summary of what I understand

#### Chapter 3:

moving into the chapter 3 of the rust book, we have Variables and mutability and studying through I was able to understand what Variables are and how they are being written in rust and also the beauty of mutability.

In rust, variables are written using the `let` or `const` keyword and the variable name which are written in snake case. While declaring a variable in rust, we use the `mut` keyword to tell the compiler that the value can be changed but by default all variables are immutable by default which is also the case of a constant as well as there are immutable and can be use in any scope of the project.

As we moved on we also encountered shadowing and how rust handles shadowing… in rust a variable that has been previously declared can be redeclared with the same variable name as the previously declared variable with the same or different data type.

As we all know Variables are used to store values and the values could be of any types which let us move to the next sub topic which is the Data Types.

In rust we have two data types basically,

1. Scaler type
2. Compound type

**Scaler Types:**
there are four types of scaler types and why are they scaler types is because it is of a single value.
the four types of the scaler types are:

1. Integer type (Signed and Unsigned)
2. Floating type
3. char type
4. boolean type (true or false)

**Compound type:**
we have two types of the compound types:

1. Tuple
2. Array

**Function:**
functions are reusable lines of instructions to perform specific task. In rust, function declaration starts with the keyword `fn` then the function name, parenthesis and curly brackets (which are the scope of the function). In rust, there two types of functions,

1. Expression
2. Statement

---

### Chapter 2:

The chapter 2 shows us about how to create a cargo project and also give us introduction to how rust works and the literals. The chapter 2 opens us to the rust library of the input and output and how we can interact with a project using the cli. The chapter 2 also introduces us to the Package manager the Cargo.toml where we have us import other crates into our project for use.

---

### ❓ What I do not understand

#### Chapter 3:

while reading this chapter, in the function sub topic, I learnt about Statement and Expression and yes, I know expression returns a value now while practicing, an expression function can return with or without return keyword but the most shocking thing is that after returning the value without the return keyword, we can continue writing many other set of instruction, why does rust allow this?

#### Chapter 2:

while studying further, I came across Ok and Err variants and could not figure why there are in the control flow.
