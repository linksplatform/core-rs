[![Crates.io](https://img.shields.io/crates/v/platform-data.svg?label=crates.io&style=flat)](https://crates.io/crates/platform-data)
[![Docs.rs](https://docs.rs/platform-data/badge.svg)](https://docs.rs/platform-data)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://github.com/linksplatform/core-rs/blob/main/LICENSE)

# [Data](https://github.com/linksplatform/core-rs)

LinksPlatform's Platform.Data Class Library for Rust.

This crate provides core data types and traits for the [Links Platform](https://github.com/linksplatform) — an associative data storage system. It defines the fundamental abstractions for working with links (doublets) in Rust.

## Overview

The `platform-data` crate provides:

- **`LinkType`** — A trait defining the numeric types that can be used as link identifiers
- **`Links`** — The core trait for CRUD operations on doublet links storage
- **`Flow`** — Control flow type for iteration operations (Continue/Break)
- **`Query`** — A wrapper for link queries using copy-on-write semantics
- **`Point`** — A structure representing a repeating element
- **`Hybrid`** — A type for handling internal and external link references
- **`LinksConstants`** — Configuration constants for links storage
- **`AddrToRaw` / `RawToAddr`** — Converters between address and raw representations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
platform-data = "0.1.0-beta.3"
```

## Usage

### Using the `query!` macro

```rust
use platform_data::{query, Query, ToQuery};

// Create queries using the macro
let q: Query<usize> = query![1, 2, 3];
assert_eq!(q.len(), 3);

// Empty query
let empty: Query<usize> = query![];
assert!(empty.is_empty());
```

### Working with Flow control

```rust
use platform_data::Flow;

let mut collected = vec![];

// Use Flow with try_for_each by converting to ControlFlow
(0..20).try_for_each(|i| {
    collected.push(i);
    if i == 10 { Flow::Break.into_control_flow() } else { Flow::Continue.into_control_flow() }
});

assert_eq!(collected, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

// Or use Flow directly with manual iteration
let mut collected2 = vec![];
for i in 0..20 {
    collected2.push(i);
    let flow = if i == 10 { Flow::Break } else { Flow::Continue };
    if flow.is_break() {
        break;
    }
}
assert_eq!(collected2, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
```

### Using Point for repeated elements

```rust
use platform_data::Point;

let point = Point::new(42, 5);

assert_eq!(point.len(), 5);
assert_eq!(point.get(0), Some(&42));
assert_eq!(point.get(4), Some(&42));

// Iterate over repeated elements
for value in point {
    assert_eq!(value, 42);
}
```

### Address conversion with Hybrid

```rust
use platform_data::{AddrToRaw, RawToAddr, Hybrid};

let addr: usize = 100;

// Convert address to raw and back
let raw = AddrToRaw.convert(addr);
let restored = RawToAddr.convert(raw);

assert_eq!(restored, addr);

// Hybrid values
let internal = Hybrid::internal(42usize);
assert!(internal.is_internal());

let external = Hybrid::external(42usize);
assert!(external.is_external());
```

### Implementing the Links trait

```rust
use platform_data::{Links, LinkType, LinksConstants, Flow, Error, ReadHandler, WriteHandler};

// The Links trait provides CRUD operations for doublet links:
// - constants_links() - get storage configuration
// - count_links(query) - count matching links
// - create_links(query, handler) - create new links
// - each_links(query, handler) - iterate over matching links
// - update_links(query, replacement, handler) - update existing links
// - delete_links(query, handler) - delete links
```

## API Reference

### Core Types

| Type | Description |
|------|-------------|
| `LinkType` | Trait bound for numeric types usable as link identifiers (unsigned integers) |
| `Links<T>` | Main trait defining CRUD operations for links storage |
| `Flow` | Control flow enum: `Continue` or `Break` for iteration control |
| `Query<'a, T>` | Copy-on-write query wrapper for efficient link queries |
| `Point<T>` | Structure representing a single value repeated multiple times |
| `Hybrid<T>` | Type for distinguishing internal and external link references |
| `LinksConstants<T>` | Configuration constants including null, any, continue, break, etc. |
| `Error<'a, T>` | Error type for links operations |

### Type Aliases

| Alias | Description |
|-------|-------------|
| `ReadHandler<'a, T>` | Handler for read operations: `&mut dyn FnMut(&[T]) -> Flow` |
| `WriteHandler<'a, T>` | Handler for write operations: `&mut dyn FnMut(&[T], &[T]) -> Flow` |

### Converters

| Type | Description |
|------|-------------|
| `AddrToRaw` | Converts link address to raw representation |
| `RawToAddr` | Converts raw representation back to link address |

## Requirements

This crate requires **Rust 1.79 or later** (stable toolchain). The `associated_type_bounds` feature used for `Error:` bounds was stabilized in Rust 1.79.

## Dependencies

- [beef](https://crates.io/crates/beef) — Faster and more compact Cow implementation
- [funty](https://crates.io/crates/funty) — Fundamental type unification
- [thiserror](https://crates.io/crates/thiserror) — Derive macro for error types

## Related Projects

- [mem-rs](https://github.com/linksplatform/mem-rs) — Memory management for Links Platform
- [trees-rs](https://github.com/linksplatform/trees-rs) — Tree structures for Links Platform
- [Data.Doublets](https://github.com/linksplatform/Data.Doublets) — C# implementation of doublet links

## Support

Ask questions at [stackoverflow.com/tags/links-platform](https://stackoverflow.com/tags/links-platform) (or with tag `links-platform`) to get our free support.

You can also get real-time support on [our official Discord server](https://discord.gg/eEXJyjWv5e).

## License

This project is licensed under the [Unlicense](LICENSE) — released into the public domain.
