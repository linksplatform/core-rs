---
bump: major
---

### Changed
- Migrated from nightly Rust to **stable Rust** toolchain (requires Rust 1.79+)
- Removed all unstable feature flags:
  - `try_trait_v2` - Flow no longer implements `Try` trait
  - `type_alias_impl_trait` - Point now uses explicit `PointIter` type
  - `const_trait_impl`, `const_convert`, `const_deref`, `const_refs_to_cell`, `const_result_drop` - LinkType/FuntyPart traits are no longer const
  - `step_trait` - LinkType no longer requires `Step` bound
  - `associated_type_bounds` - still used but stabilized in Rust 1.79
- `Flow` type changes:
  - Added `into_control_flow()` method for use with `try_for_each`
  - Removed `Try` and `FromResidual` trait implementations (nightly-only)
- `Point` type changes:
  - Added explicit `PointIter` iterator type (publicly exported)
- `LinkType` trait changes:
  - Removed `Step` trait bound
  - Removed `const` from trait and impl
- `FuntyPart` trait changes:
  - Simplified implementation without const generics
  - Now uses `expect()` instead of `unreachable_unchecked()`
- Updated CI/CD pipeline to use `dtolnay/rust-toolchain@stable`

### Fixed
- Crate now compiles on stable Rust without any feature flags
