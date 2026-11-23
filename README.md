# rscp

Rust template for competitive programming by Tobiasz Perczak. Fast input and output
macros live in `src/lib.rs`, and every build bundles the lib and your solution into a
single `send.rs` that you submit.

## Quick start

```
cargo install cargo-generate
cargo generate --path /abs/path/to/rscp --name problem_a
cd problem_a
```

Write the solution in `src/main.rs`, drop samples into `tests/` and run `cargo cptest`.
Submit `send.rs` from the project root.

```rust
use rscp::{get, get_vec, load_input, putln};

fn main() {
    load_input!();
    let n = get!(usize);
    let v = get_vec![i64; n];
    putln!("{}", v.iter().sum::<i64>());
}
```

## API

Input (call `load_input!()` first):

| Item | Description |
| --- | --- |
| `get!()` / `get!(T)` | Next value; type from context or explicit. Numbers, `String`, `char`, `bool`, tuples up to 6 |
| `get_vec![T; n]` | `n` values of `T` (tuples and `_` allowed) |
| `get_line()` | Rest of the current line, or the next line if the rest is blank |
| `get_bytes()` | Next token as `&'static [u8]`, no allocation |
| `get_graph(n, m)` / `get_digraph(n, m)` | Edges `a b` (1-indexed), 0-indexed adjacency lists |
| `get_wgraph::<W>(n, m)` / `get_wdigraph::<W>(n, m)` | Edges `a b w`, lists of `(to, w)` |
| `get_tree(n)` / `get_wtree::<W>(n)` | Same as above with `n - 1` edges, undirected |

Output (buffered, flushed automatically when `main` ends):

| Item | Description |
| --- | --- |
| `put!` / `putln!` | Like `print!` / `println!` |
| `putv!(it)` / `putv!(it, sep)` | Print an iterable separated by spaces (or `sep`), then a newline |
| `yn!(c)` / `yn!(c, y, n)` | Print `YES` / `NO` (or custom words) |
| `flush()` | Flush manually (interactive mode) |

## Interactive problems

Use `load_interactive!()` instead of `load_input!()`. Input is then read line by line
as needed, and `putln!` flushes after every line. After `put!` call `flush()` yourself.

```rust
use rscp::{get, load_interactive, putln};

fn main() {
    load_interactive!();
    let (mut lo, mut hi) = (1u64, get!(u64));
    while lo < hi {
        let mid = (lo + hi) / 2;
        putln!("? {}", mid);
        if get!(u8) == 1 { hi = mid } else { lo = mid + 1 }
    }
    putln!("! {}", lo);
}
```

`check.sh` only feeds files through stdin, so it cannot test a real interactor.

## Testing

Put cases in `tests/` as `name.in` and `name.out` pairs, then run:

```
cargo cptest                  # 1 s, 256 MB
TL=2 ML=64 cargo cptest       # custom limits
./tools/check.sh 2 64         # same thing without cargo
```

Each case is run against the bundled `send.rs` and reported as `ok`, `WA`, `TLE`, `MLE`
or `RE`. Output is compared ignoring whitespace and blank lines. Memory is peak RSS and
needs the `time` package (`sudo pacman -S time`).

## Layout

```
Cargo.toml
send.rs        generated on every build, this is what you submit
src/lib.rs     the template
src/main.rs    your solution
tests/         *.in / *.out cases
tools/
  build.rs     bundles lib.rs + main.rs into send.rs
  check.rs     cargo test entry point
  check.sh     test runner
```

## How bundling works

`tools/build.rs` writes `send.rs` as the contents of `lib.rs` (with the `pub ` prefix
removed from top-level items) followed by `main.rs` (without its `use <crate>::...;`
line). So:

- Keep the import of the lib on a single line: `use rscp::{get, load_input};`
- Top-level items in `lib.rs` must start at column 0 for the `pub` stripping to work.
- Everything ends up in one module, so avoid naming your own items `token`, `refill`,
  `flush`, `get_line` and so on.

## Notes

- Malformed or missing input is undefined behavior (`unwrap_unchecked`).
- Do not mix `println!` with `putln!`; the buffered output would come out of order.
- The output is not flushed by `std::process::exit`.
- `load_input!` leaks the input buffer on purpose, which gives `&'static str` tokens.
