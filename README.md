# unwind\_aborts

Prevent your panics from unwinding past FFI boundaries with this one simple trick!

Designed to be used in place of [`#[unwind(aborts)]`][unwind_aborts_upstream]
until it is stabilized.

## Usage

Add this to your `[dependencies]` in `Cargo.toml`:

```toml
unwind_aborts = "0.1.0"
```

Annotate your functions with `#[unwind_aborts]` to catch stack unwinding and
abort the process instead:

```rust
use unwind_aborts::unwind_aborts;

#[unwind_aborts]
pub extern fn foo() {
    panic!("this is safe");
}
```

The example above is equivalent to:

```rust
pub extern fn foo() {
    let result = std::panic::catch_unwind(|| {
        panic!("this is safe");
    });
    match result {
        Ok(x) => x,
        Err(_) => std::process::abort(),
    }
}
```

[unwind_aborts_upstream]: https://github.com/rust-lang/rust/issues/58760
