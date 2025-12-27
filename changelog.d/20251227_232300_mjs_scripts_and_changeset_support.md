---
bump: minor
---

### Changed
- Migrated all CI/CD scripts from Python to JavaScript ES modules (.mjs) for enhanced performance
- Updated release workflow to use Node.js 20.x for script execution
- Added automatic version bumping based on changelog fragment frontmatter

### Added
- New `get-bump-type.mjs` script that parses changelog fragments and determines version bump type
- Frontmatter support in changelog fragments with `bump: major|minor|patch` specification
- Automatic version bumping during release based on highest priority bump type from fragments

### Documentation
- Updated `changelog.d/README.md` with comprehensive frontmatter documentation and examples
- Updated `CONTRIBUTING.md` with new script references and fragment format instructions
