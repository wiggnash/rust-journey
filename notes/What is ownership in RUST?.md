# Ownership in Rust

## Preface to Ownership

This is the **unique feature of Rust** compared to other languages. This is what makes Rust **memory safe** without a [[Garbage Collector]].

**Key Points:**

- Memory safety is achieved at **compile time** through ownership rules
- The compiler figures out exactly when memory should be freed
- **No manual free calls needed**
- **No garbage collector running at runtime**
- **No performance overhead** because of garbage collector

Whenever memory goes out of scope, the Rust compiler will **automatically clean up** the memory used by the variable.

---

## What is Ownership?

Ownership is a **set of rules** that tells the Rust program how to manage memory.

Rust memory is managed through a **system of ownership** — a set of rules that the compiler checks. If the rules are violated, the program will **not compile**.

> 💡 **Key Insight:** Ownership will not slow down the program while it is running.

### Main Purpose

The main purpose of ownership is to **manage heap data**.

📌 **Related:** [[The Stack and Heap Memory]]

### Problems Ownership Addresses

Ownership solves three critical memory management problems:

1. **Tracking heap usage** — Keeping track of what parts of code are using what data on the heap
2. **Minimizing duplicate data** — Reducing the amount of duplicate data on the heap
3. **Cleaning up unused data** — Removing unused data on the heap so we don't run out of space