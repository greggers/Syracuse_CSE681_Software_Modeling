# Module 03: Type Safety Demonstration

This module demonstrates the differences between type-safe and type-unsafe programming languages through practical examples comparing C++ and Rust.

## Overview

Type safety is a crucial aspect of programming language design that prevents certain categories of programming errors at compile time or runtime. This demonstration contrasts:

- **C++**: A systems programming language with manual memory management and potential type unsafe operations
- **Rust**: A systems programming language designed with memory safety and type safety as core principles

## Examples

### 1. Buffer Overflow Protection
- **`buffer_overflow.cpp`**: Demonstrates how C++ allows dangerous buffer overflows
- **`buffer_safe.rs`**: Shows how Rust prevents buffer overflows at compile time

### 2. Use After Free Prevention
- **`use_after_free.cpp`**: Shows dangerous memory access after deallocation in C++
- **`memory_safe.rs`**: Demonstrates Rust's ownership system preventing use-after-free

### 3. Null Pointer Safety
- **`null_pointer.cpp`**: C++ allows dangerous null pointer dereferences
- **`option_safe.rs`**: Rust's Option type system eliminates null pointer exceptions

### 4. Data Race Prevention
- **`data_race.cpp`**: Concurrent access issues possible in C++
- **`thread_safe.rs`**: Rust's ownership system prevents data races at compile time

## Key Learning Points

1. **Compile-time vs Runtime Safety**: How different languages catch errors at different stages
2. **Ownership Systems**: Rust's unique approach to memory management
3. **Type System Design**: Impact of language design on program safety
4. **Performance vs Safety**: How modern languages achieve both

## Building and Running

### C++ Examples
```bash
g++ -o buffer_overflow buffer_overflow.cpp
g++ -o use_after_free use_after_free.cpp
g++ -o null_pointer null_pointer.cpp
g++ -pthread -o data_race data_race.cpp
```

### Rust Examples
```bash
cargo run --bin buffer_safe
cargo run --bin memory_safe
cargo run --bin option_safe
cargo run --bin thread_safe
```

Note: Some Rust examples will not compile due to safety violations - this is the intended demonstration of the language's protective features.
