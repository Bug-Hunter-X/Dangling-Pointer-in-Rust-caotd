# Dangling Pointer Bug in Rust

This repository demonstrates a common error in Rust: creating and using a dangling pointer.  The code attempts to access memory after it has been freed, leading to undefined behavior and potential crashes.

## The Bug

The `bug.rs` file contains the problematic code. It creates a vector, gets a raw pointer to its data, and then drops the vector.  Subsequently, it attempts to dereference the raw pointer, which now points to deallocated memory. This results in undefined behavior.

## The Solution

The `bugSolution.rs` file demonstrates a corrected version. It avoids the dangling pointer by ensuring the vector remains in scope while the pointer is used.  Alternatively, using `&vec[0]` provides safe memory access.