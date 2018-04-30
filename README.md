# Weedle - A WebIDL Parser

Parses valid WebIDL definitions & produces a data structure starting from
[`Definitions`](https://docs.rs/weedle/0.1.1/weedle/struct.Definitions.html).

### Basic Usage

In Cargo.toml
```
[dependencies]
weedle = "0.1.1"
```

Then, in `src/main.rs`
```rust
extern crate weedle;

fn main() {
    let parsed = weedle::parse("
        interface Window {
            readonly attribute Storage sessionStorage;
        };
    ").unwrap();
    println!("{:?}", parsed);
}
```