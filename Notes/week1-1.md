### Day 1 Notes - Rust Intro~
#### 4/1/2024

- `Cargo new` command will setup rust a project for you
  - Cargo.toml is the config file that is generated
    - Edition="2021" specifies the version, but more importantly tells the compiler that you aren't using the classic version of rust.

- Rust is type inferred.
  - There are still explicit type declarations if desired.
- All rust numeric types have specified size, there is no standard int.
  - u8, u16, u32, u64, u128 (unsigned)
  - i8, i16, i32, i64, i128 (signed)
  - There is no type promotion (e.g. i8 to i16).
- [Rust Unit type](https://doc.rust-lang.org/std/primitive.unit.html)
- Variable creation in rust is with the `let` keyword.
- Variable mutation has to be enabled at creation with `let mut`.
- `Cargo clippy file-name.rs` will give you diffs to fix your program (cool!)
- Use cargo clippy for all assignments before turning them in.
  - Use cargo clippy over cargo check in vs code
  - Run cargo clippy and cargo format on your assignments before submission.
- In for loops the range is declared with `start .. end`.
  - `start ..= end` is inclusive on both sides.
- 