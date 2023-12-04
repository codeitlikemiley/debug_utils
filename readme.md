# debug! macro
A macro for printing debug information about variables.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
debug_utils = "0.1.0"
```

## Usage

```rust
use debug_utils::debug;
fn main() {
    let x = 42;
    debug!(x);
}
```

Outputs: {var} <{type}> = {value}
```shell
{x} <i32> = 42
```

