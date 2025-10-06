
# BigNum

> A small Rust crate providing arbitrary-precision numeric types, parsing/encoding across bases,
> and a lightweight expression evaluator / calculator example.

## What this repository contains

- `src/bigNum.rs` — core BigNum implementation (arbitrary-precision numeric type).
- `src/codec.rs` — parsing and encoding helpers (supports multiple bases).
- `src/AST.rs` — abstract syntax tree for the calculator/evaluator.
- `src/main.rs` — example CLI / calculator demonstrating usage.
- `src/lib.rs` — library entry point.
- `src/tests.rs` — unit tests for numeric operations and utilities.
- `build.rs` — build helper (parser generation with LALRPOP).

## Quick start

Build the project (release):

```powershell
cargo build --release
```

Run the example CLI:

```powershell
cargo run --release
```

Run tests:

```powershell
cargo test
```

Run the bundled examples (if present in `examples/`):

```powershell
# Run the arithmetic example
cargo run --example arithmetic

# Run the encode example
cargo run --example encode

# Run the more complex scale example
cargo run --example complex_scale
```

## Library usage

You can use this crate as a library in other projects or call the example CLI. Example (pseudo-code):

```rust
use big_num::BigNum;
use codec::{parse, encode, Base};

let a = parse("123.45", Base::Decimal).unwrap();
let b = BigNum::from(1u32);
let sum = a + b;
let s = encode(&sum, Base::Hexadecimal);
```

Check the source files in `src/` for exact function and type names.

## Notes on internals

- `BigNum` stores sign, coefficient and exponent (see `src/bigNum.rs`).
- `codec.rs` contains helpers for parsing/encoding in Binary/Octal/Decimal/Hex (and possibly others).
- The example CLI uses an LALRPOP-generated parser (see `build.rs` and `calculator.lalrpop`).

## Contributing

If you'd like to contribute:

1. Fork the repository and create a feature branch.
2. Add tests for new behavior in `src/tests.rs`.
3. Run `cargo test` and `cargo fmt` locally.
4. Open a pull request describing your change.

## License

No license file is present in this repository. If you plan to publish this crate, add a `LICENSE` file and update `Cargo.toml` accordingly.

## Contact / Maintainer

Repository owner: ngerow2025

---

If you want a shorter or differently styled README (badges, examples, API docs, or publishing instructions), tell me what you'd like and I will update it.

## Examples

Below are short, copy-pasteable examples that show common usage patterns. You can also run the corresponding example binaries in `examples/` (if present) with `cargo run --example <name>`.

1) Parse and print a decimal value

```rust
use big_num::{parse, Base};

let a = parse("123.45", Base::Decimal);
println!("parsed: {}", a);
```

2) Basic arithmetic (add/sub/mul/div)

```rust
use big_num::{parse, BigNum};

let a = parse("100", big_num::Base::Decimal);
let b = BigNum::from(7u32);
println!("{} + {} = {}", a, b, a.clone() + b.clone());
```

3) Encode to different bases

```rust
use big_num::{parse, encode, Base};

let n = parse("255.5", Base::Decimal);
println!("hex: {}", encode(n.clone(), Base::Hexadecimal));
println!("base64: {}", encode(n, Base::Base64));
```

4) More complex: scale a fraction by a large power and encode

```rust
use big_num::{BigNum, encode, Base};

let frac = BigNum::from(1u32) / BigNum::from(3u32);
let two_pow_100 = BigNum::from(2u32).pow(BigNum::from(100u32));
let scaled = frac * two_pow_100;
println!("scaled (base64) = {}", encode(scaled, Base::Base64));
```