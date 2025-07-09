# Rust Ownership Summary

## What is Ownership?

Ownership is a way to manage memory in Rust.

## Stack vs Heap

**Stacks**: It is about last-in-first-out. It's all about the last function call or variable that goes in is the first one that comes out.

**Heaps**: Heaps takes up a space, registers it as in use, and then returns a pointer to the address. Another point, accessing data on the heap is slower than on the stack because you have to follow the pointer to get there.

## Ownership Rules

1. Each value has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

In ownership, the variable scope is being implemented where a scope is the range within a program for which an item is valid.

## String Type

To manage memory in Rust, we use the string type not string literals. For example:

```rust
let S = String::from("hello");
```

This way, this kind of string can be mutated. To change or modify a string after creating it, the string has to be declared as mutable.

## Memory Management

The way Rust handles memory allocation and garbage collection is by the variable scope, where allocation of memory starts at the beginning of the definition and ends at where the use of the function ends. Instead of having to free memory like other languages, Rust uses a function called drop.

## Copy vs Move

A variable can be copied or moved, e.g.:

```rust
let x = 5;
let y = x;
```

In this case, both x and y are 5 and this is because y copied the binded value of x.

But in the case of a string, we have:

```rust
let S1 = String::from("hello");
let S2 = S1;
```

In this case, we have what other languages call a shallow copy to make this happen, which is called move in Rust, where S2 copied the pointer capacity and length of S1. This means S1 is no longer usable after the move - Rust does this to prevent having two variables trying to free the same memory.

If we want to copy S1 to S2 properly, we use the dot clone method:

```rust
let S2 = S1.clone();
```

The reason why we do not clone on integers is because integers have a definite size and are stored on stack.

## Referencing and Borrowing

A reference is like a pointer or an address pointing to the value of a stored data. To use that, we call it a `&variable` (using the ampersand symbol).

For mutable references, you can only have one mutable reference to a piece of data at a time. You also can't have both mutable and immutable references to the same data at the same time. If you want to use a mutable reference, make sure any other references end their scope first, or create the mutable reference in a different scope.

## Dangling References

Dangling references happen when you have a reference pointing to memory that has been freed or is no longer valid. Rust will not let this happen - it will catch this at compile time and throw an error. The best way to solve a dangling reference is to return the actual data directly instead of trying to return a reference to local data.
