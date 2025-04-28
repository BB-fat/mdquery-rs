# mdquery-rs

A Rust binding library for macOS Spotlight search. Utilizes macOS Metadata Query API to perform efficient file searches.

## Features

- Support for macOS Spotlight search queries
- Clean Rust API interface
- Support for various query conditions: file name, content type, modification time, file size, etc.
- Custom search scopes (home directory, entire computer, network, etc.)
- Builder pattern for constructing complex queries
- Complete error handling and type safety

## System Requirements

- macOS operating system
- Rust 1.56.0 or higher

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
mdquery-rs = "0.1.0"
```

## Usage Examples

### Basic Usage

```rust
use mdquery_rs::{MDQueryBuilder, MDQueryScope, MDQueryCompareOp, MDItemKey};

// Find PDF files in home directory with "document" in their name
let query = MDQueryBuilder::default()
    .name_like("document")
    .extension("pdf")
    .time(
        MDItemKey::ModificationDate,
        MDQueryCompareOp::GreaterThan,
        chrono::Utc::now().timestamp() - 86400 * 30, // Within 30 days
    )
    .build(vec![MDQueryScope::Home], Some(20))
    .unwrap();

let results = query.execute().unwrap();
for item in results {
    println!("File: {:?}, Display name: {:?}", item.path(), item.display_name());
    
    // Get all metadata attribute names
    let attr_names = item.get_attribute_names();
    println!("Available attributes: {:?}", attr_names);
}
```

### Searching Applications

```rust
use mdquery_rs::{MDQueryBuilder, MDQueryScope};

// Find applications with "Safari" in their name
let query = MDQueryBuilder::default()
    .name_like("Safari")
    .is_app()
    .build(vec![MDQueryScope::Computer], Some(5))
    .unwrap();

let results = query.execute().unwrap();
for item in results {
    println!("Application: {:?}", item.path());
    println!("Display name: {:?}", item.display_name());
}
```

## Contributing

Contributions and issue reports are welcome!

## License

MIT

## Related Projects

- [objc2](https://github.com/madsmtm/objc2) - Rust bindings to the Objective-C runtime and frameworks

## TODO

- [x] Implement async query API to provide non-blocking search capabilities
- [ ] Add predefined common query templates
- [ ] Provide specialized search methods for common file types (images, documents, audio, etc.)
- [ ] Optimize performance for batch metadata reading
