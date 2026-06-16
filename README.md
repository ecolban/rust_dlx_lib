# rust-dlx-lib

`rust-dlx-lib` is a small Python extension module (implemented in Rust with `pyo3`) that solves **exact cover** problems using **Dancing Links (DLX)**.

It exposes one Python function:

```python
solve_dlx(columns: list[bool], rows: list[list[int]], clues: list[int] | None = None) -> list[int]
```

The return value is a list of row indices forming one exact cover solution, or an empty list when no solution is found.

## What this library solves

DLX solves exact cover instances represented as a binary matrix:

- each row is a candidate choice
- each column is a constraint
- selected rows must cover every **primary** constraint exactly once
- **secondary** constraints are optional and may be covered zero or one time

You choose which columns are primary by passing `columns` as booleans.

## API

### `solve_dlx(columns, rows, clues=None)`

- `columns: list[bool]`
  - One entry per matrix column.
  - `True` means primary constraint, `False` means secondary constraint.
- `rows: list[list[int]]`
  - Binary matrix rows (`0`/`1`) with the same width as `columns`.
- `clues: list[int] | None`
  - Optional list of preselected row indices.
  - Useful when part of the solution is already fixed.

Returns:

- `list[int]`: row indices included in one solution.
- `[]` if no solution exists.

## Prerequisites

Because this package builds a Rust extension, the machine installing it needs:

- Python 3.10+
- `rustc` and `cargo` (via Rust toolchain)

### Install Rust (`rustc` + `cargo`)

The recommended installer is `rustup`.

macOS / Linux:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Windows (PowerShell):

```powershell
winget install --id Rustlang.Rustup
```

If `winget` is not available, install from `https://rustup.rs`.

### Verify installation

```bash
rustc --version
cargo --version
```

If those commands fail, ensure your shell has Cargo's bin directory on `PATH` (typically `$HOME/.cargo/bin`).

## Using this library from another repo

If your app lives in another repo, you can install this package as a local path dependency.

You do **not** need to run `maturin` manually in `rust_dlx_lib` when consuming it this way.

### Option A: Pipenv (`Pipfile`)

In the consumer repo's `Pipfile`, add:

```toml
[packages]
rust-dlx-lib = { path = "path/to/rust_dlx_repo", editable = true }
```

Then install/update dependencies from the consumer repo:

```bash
cd /path/to/consumer-repo
pipenv install
```

### Option B: `requirements.txt`

In the consumer repo's `requirements.txt`, add one of these lines:

```text
-e path/to/rust_dlx_repo
```

or (non-editable):

```text
path/to/rust_dlx_repo
```

Then install from the consumer repo:

```bash
cd /path/to/consumer-repo
python -m pip install -r requirements.txt
```

### Quick check (from the consumer repo environment)

```bash
python -c "from rust_dlx_lib import solve_dlx; print(callable(solve_dlx))"
```

## Usage example

```python
from rust_dlx_lib import solve_dlx

# 4 primary columns
columns = [True, True, True, True]

# Binary matrix (rows x columns)
rows = [
	[1, 0, 1, 0],
	[1, 0, 0, 1],
	[0, 1, 1, 0],
	[0, 1, 0, 1],
]

solution = solve_dlx(columns, rows)
print(solution)  # Example output: [0, 3]
```

With clues:

```python
# Force row 0 to be part of the solution
solution = solve_dlx(columns, rows, clues=[0])
```

### Extended example -  Sudoku solver

Sudoku is a classic exact cover problem. A 9×9 grid has **324 constraints** (four groups of 81):

1. **Cell** — every cell contains exactly one digit
2. **Row** — every row contains each digit exactly once
3. **Column** — every column contains each digit exactly once
4. **Box** — every 3×3 box contains each digit exactly once

Each of the 729 possible placements (row, col, digit) becomes a matrix row covering exactly one constraint from each group. Pre-filled cells become `clues`.

```python
from rust_dlx_lib import solve_dlx


def solve_sudoku(grid):
    """
    Solve a 9x9 Sudoku puzzle using DLX.
    grid: 9x9 list of lists (0 = empty, 1-9 = given digit).
    Returns a solved 9x9 grid, or None if unsolvable.
    """

    # 324 primary constraints
    columns = [True] * (4 * 9 * 9)

    def make_rows() -> list[list[int]]:
        row_length = 4 * 9 * 9
        offsets = list(range(0, row_length, 9 * 9))

        def make_row(row, col, val):  # val is 0-based
            box = row - row % 3 + col // 3
            res = [0] * row_length
            res[offsets[0] + row * 9 + col] = 1  # cell constraint
            res[offsets[1] + row * 9 + val] = 1  # row constraint
            res[offsets[2] + col * 9 + val] = 1  # column constraint
            res[offsets[3] + box * 9 + val] = 1  # box constraint
            return res

        # 9 * 9 * 9 candidate placements; index = r * 9 * 9 + c * 9 + d  (d is 0-based)
        return [make_row(r, c, v) for r in range(9) for c in range(9) for v in range(9)]

    def row_idx(r, c, d):
        """The index of the row representing the placement of d to grid row r and grid column c."""
        return ((r * 9) + c) * 9 + (d - 1)

    # Pre-filled cells map directly to row indices
    clues = [row_idx(r, c, d)
             for r, row in enumerate(grid)
             for c, d in enumerate(row) if d != 0]

    solution = solve_dlx(columns, make_rows(), clues=clues)
    if not solution:
        return None

    solution.sort()
    return [[solution[9 * r + c] % 9 + 1 for c in range(9)] for r in range(9)]


# "AI Escargot" by Arto Inkala (2006) — one of the hardest puzzles ever published
# @formatter:off
puzzle = [
    [1, 0, 0,  0, 0, 7,  0, 9, 0],
    [0, 3, 0,  0, 2, 0,  0, 0, 8],
    [0, 0, 9,  6, 0, 0,  5, 0, 0],

    [0, 0, 5,  3, 0, 0,  9, 0, 0],
    [0, 1, 0,  0, 8, 0,  0, 0, 2],
    [6, 0, 0,  0, 0, 4,  0, 0, 0],

    [3, 0, 0,  0, 0, 0,  0, 1, 0],
    [0, 4, 0,  0, 0, 0,  0, 0, 7],
    [0, 0, 7,  0, 0, 0,  3, 0, 0],
]
# @formatter:on

result = solve_sudoku(puzzle)
if result:
    for grid_row in result:
        print(grid_row)
```

Expected output:

```
[1, 6, 2, 8, 5, 7, 4, 9, 3]
[5, 3, 4, 1, 2, 9, 6, 7, 8]
[7, 8, 9, 6, 4, 3, 5, 2, 1]
[4, 7, 5, 3, 1, 2, 9, 8, 6]
[9, 1, 3, 5, 8, 6, 7, 4, 2]
[6, 2, 8, 7, 9, 4, 1, 3, 5]
[3, 5, 6, 4, 7, 8, 2, 1, 9]
[2, 4, 1, 9, 3, 5, 8, 6, 7]
[8, 9, 7, 2, 6, 1, 3, 5, 4]
```

## Notes

- The solver returns the first solution it finds.
- Invalid clues (row indices that do not exist) raise an error.
- All rows must have the same length as `columns`.

## Project layout

- `src/lib.rs`: Python module bindings (`pyo3` entrypoint)
- `src/dlx.rs`: core Dancing Links implementation
- `Cargo.toml`: Rust crate metadata
- `pyproject.toml`: Python packaging/build config (`maturin`)

