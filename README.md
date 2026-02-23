# rust_list

A small Rust project implementing a stack-style linked list, used to explore ownership, `Box`, and custom `Drop`.

## Project structure

- **listversion1** — First version: `List` with `push` / `pop`, heap-allocated nodes via `Box<Node>`, and a manual `Drop` impl to avoid recursive drop and stack overflow.
- **listversion1/explain/** — Notes and explanations (e.g. why we need a custom `Drop`); see [explain/drop.md](listversion1/explain/drop.md).

## Requirements

- [Rust](https://www.rust-lang.org/) (e.g. via rustup)

## Build and run

From the repo root:

```bash
# Build the workspace
cargo build

# Run the listversion1 binary
cargo run -p listversion1

# Run tests
cargo test -p listversion1
```

## What’s in listversion1

- **`List`** — Wraps a `Link` (either `Empty` or `More(Box<Node>)`).
- **`push(elem)`** — Adds an element at the head using `mem::replace` to swap the current head into the new node.
- **`pop()`** — Removes and returns the head element as `Option<i32>`.
- **`Drop`** — Implemented by hand so dropping a long list doesn’t recurse through every node and overflow the stack; see [listversion1/explain/drop.md](listversion1/explain/drop.md) for the full explanation.

## License

Unlicensed / your choice.
