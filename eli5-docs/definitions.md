# Definitions

## Traits

Traits are shared behavior you want to give across different objects/structs.

## Attributes

Attributes are metadata. e.g.

```rust
// This is a metadata that is applied throughout the whole file.
// It has the hash (#) and a bang (!) included.
#![allow(dead_code)]


// This is a metadata applied to something immediately following it
// It has the hash (#) only.
#[derive(Debug)]
struct MinMax(i64, i64);
```

## Ownership

- Rust uses the concept of 'ownership' to determine whether or not to allocate or release memory.
- When you pass pointer data types (like String) into a new variable (for example, just a new statement or into a function), you will perform the `move` operation that transfers ownership of that heap data to the new variable declaration.
- To avoid the `move` operation, we use references.

## References

- References use the `&` notation to denote the 'reference to' a variable.