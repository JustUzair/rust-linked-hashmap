# HashMap

[![Rust](https://img.shields.io/badge/rust-1.89.0-000000?style=flat&logo=rust)](https://www.rust-lang.org/)
[![HashMap](https://img.shields.io/badge/HashMap-Data%20Structures-orange?style=flat)](#)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat)](#)

A low-level implementation of a dynamic hash table from scratch, covering the critical patterns and design choices that constitute the hash-map lib.

## Overview

This project reimplements Rust's standard library `HashMap` without external dependencies, covering the critical patterns that make it a powerful data structure:

- **Dynamic resizing** with load factor management to maintain O(1) average-case performance
- **Entry API** (occupied/vacant) for ergonomic in-place mutations without double lookups
- **Generic borrowing** with `Borrow<Q>` trait to enable heterogeneous lookups (e.g., `&str` keys in `HashMap<String, V>`)
- **Iterator implementation** with proper lifetime management

**Key Implementation Details:**

- Hash table with chaining collision resolution
- Dynamic resizing with load factor management
- Entry API for ergonomic mutation patterns
- Generic code using trait bounds (`Hash + Eq`, `Borrow<Q>`)
- Iterator implementation with proper lifetime management
- Comprehensive unit tests validating correctness

## Repository

[github.com/JustUzair/rust-linked-hashmap](https://github.com/JustUzair/rust-linked-hashmap)

---

## Key Design Decisions

**Chaining for collision resolution:** Each bucket stores a vector of key-value pairs, keeping the implementation straightforward while remaining performant for reasonable load factors.

**Generic borrowing with `Borrow<Q>` trait:** Allows callers to query with `&str` while storing `String` keys, reducing unnecessary allocations. This is a powerful pattern that bridges the gap between owned and borrowed types.

**Lifetime annotations in Entry types:** The entry structs hold references back into the hashmap, enabling safe in-place operations without compromising memory safety. This pattern showcases how lifetimes and the borrow checker work in concert to prevent common mutation issues.

## Testing

Comprehensive unit tests cover insertion, retrieval, removal, and iteration across various scenarios. Tests validate correctness of load factor-driven resizing and proper handling of duplicate keys.

```bash
cargo test
```

## Running Examples

The project includes examples demonstrating common hashmap patterns:

```bash
cargo run --example std-1
cargo run --example std-2
cargo run --example std-3
cargo run --example std-4
```

---
