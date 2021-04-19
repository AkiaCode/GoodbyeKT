# GoodbyeKT
Rust library that can be reset if you think it's slow


### Example
```rust
fn main() {
    let status = goodbye_kt::reset();

    println!("{}", status);
}
```