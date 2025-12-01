## Advent of Code 2025 – Rust Solutions

This repository contains my solutions to **Advent of Code 2025**, implemented in Rust.

The project is structured as a small Cargo workspace with:

- `runner`: a single binary that runs any day/part with a consistent CLI.
- `solvers`: a library crate that contains all per-day solutions under `src/days/`.

Inputs live in the top-level `inputs/` directory.

---

## Project layout

- `Cargo.toml` – workspace definition (includes `runner` and `solvers`).
- `runner/` – binary crate; the only place with a `main` function.
  - `src/main.rs` – parses CLI arguments (via `clap`) and calls into `solvers`.
- `solvers/` – library crate with all puzzle solutions.
  - `build.rs` – auto-discovers `dayXX.rs` files in `src/days/` and generates a dispatcher.
  - `src/lib.rs` – exposes `run_day(day, part, use_example)`.
  - `src/days/`
    - `day01.rs` – implementation for Day 1 (both parts).
    - `day02.rs` 
    - `...`
- `inputs/`
  - `day01_example.txt`
  - `day01.txt`
  - `day02_example.txt`
  - `day02.txt`
  - `...`

Each day is a single module file, e.g. `solvers/src/days/day01.rs`, exposing:

- `pub fn part1(use_example: bool) -> String`
- `pub fn part2(use_example: bool) -> String`

The build script detects these automatically; you never need to edit a match statement or the workspace to add a new day.

---

## Running solutions

From the repository root:

```bash
cargo run -p runner -- <day> <part> <example|real> [expected]
```

- **`<day>`**: day number, e.g. `1`, `2`, …
- **`<part>`**: `1` or `2`.
- **`<example|real>`**:
  - `example` / `ex` – use `inputs/dayXX_example.txt`
  - `real` / `input` – use `inputs/dayXX.txt`
- **`[expected]`** (optional): if provided, the runner compares the computed answer to this value and exits with an error if they differ.

### Examples

Run Day 1, Part 1, example input, validating against the known answer `3`:

```bash
cargo run -p runner -- 1 1 example 3
```

Run Day 1, Part 2, real input (no validation):

```bash
cargo run -p runner -- 1 2 real
```

---

## Dependencies

- Rust (edition 2021)
- Crates:
  - `clap` – CLI parsing for the `runner` executable.

Install Rust via [`rustup`](https://rustup.rs/) if you don’t have it already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

