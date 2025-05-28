# metrics-utils-macros

Procedural macros for measuring function execution time and recording metrics in Rust applications.

## Features
- Attribute macro for measuring async function duration
- Attribute macro for measuring sync function duration

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
metrics-utils-macros = "0.1.0"
```

Annotate your functions:

```rust
use metrics_utils_macros::{measured_async_function, measured_function};

#[measured_async_function]
async fn my_async_fn() {
    // ...
}

#[measured_function]
fn my_fn() {
    // ...
}
```

## Documentation
See [docs.rs/metrics-utils-macros](https://docs.rs/metrics-utils-macros) for full documentation.

## License
MIT OR Apache-2.0

## Contribution
Contributions are welcome! Please open issues or pull requests on [GitHub](https://github.com/yourusername/metrics-utils-macros). # metrics-utils-macros
