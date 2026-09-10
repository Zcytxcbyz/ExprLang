# ExprLang

[![Crates.io](https://img.shields.io/crates/v/expr_lang.svg)](https://crates.io/crates/expr_lang)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/Zcytxcbyz/ExprLang/actions/workflows/ci.yml/badge.svg)](https://github.com/Zcytxcbyz/ExprLang/actions/workflows/ci.yml)

**ExprLang** is a lightweight, Rust-based **mathematical expression language** designed for scientific computing, formula evaluation, and scripting. It combines the simplicity of a calculator with the power of a scripting language.

## Features

- **Arithmetic**: `+`, `-`, `*`, `/`, parentheses, unary minus
- **Comparisons**: `<`, `<=`, `>`, `>=`, `==`, `!=`
- **Logic**: `&&`, `||`, `!` (non-zero values are truthy)
- **Variables**: dynamic typing, global scope, `let` or direct assignment
- **Data types**: numbers (`f64`), strings (with escapes), arrays (nested)
- **Control flow**: `if-elif-else` chains, `while` loops, `for-in` loops
- **Functions**: `fn name(params) = body`, supports recursion, closure capture
- **Built-in functions**: trigonometric, logarithmic, power, `max`, `min`, `len`, `concat`, `factorial`, `sign`, `is_even`, `is_odd`, `deg`, `rad`, and more
- **Arrays**: indexing and slicing (`arr[1:3]`)
- **Loop control**: `break` and `continue`
- **Comments**: single-line `#`, multi-line `/* */` (nested)
- **REPL**: interactive session for quick calculations
- **Script execution**: run `.expr` files from the command line
- **Comprehensive test suite**: unit + integration + script + CLI tests
- **Fast compilation**: builds in under 0.1 seconds
- **Cross-platform**: Linux, Windows, macOS

## Installation

### From Crates.io

```bash
cargo add expr_lang
```

### From Source

```bash
git clone https://github.com/Zcytxcbyz/ExprLang
cd ExprLang
cargo build --release
```

### Pre-built Binaries

Download the latest release from [GitHub Releases](https://github.com/Zcytxcbyz/ExprLang/releases) for your platform:

- **Linux**: `exprlang-linux-x86_64`
- **Windows**: `exprlang-windows-x86_64.exe`
- **macOS**: `exprlang-macos-x86_64`

## Usage

### REPL (Interactive)

Start the interactive REPL:

```bash
cargo run
```

Example session:

```text
ExprLang v0.3.1 (Rust Math Expression Language)
Supported: arithmetic, comparisons, logic (&&, ||, !),
strings, arrays, indexing, slicing, functions, loops, conditions.
Type 'exit' or 'quit' to exit.

> 3 + 4 * 2
11
> fn square(x) = x * x
0
> square(5)
25
> sum = 0; i = 1; while i <= 10 do (sum = sum + i; i = i + 1); sum
55
> "Hello, " + "world!"
"Hello, world!"
> exit
```

### Script Execution

Create a script file (e.g., `factorial.expr`):

```text
# Compute factorial
fn fact(n) = if n <= 1 then 1 else n * fact(n - 1);
fact(6)
```

Execute it:

```bash
cargo run -- factorial.expr
# Output: 720
```

### Step Limit

To protect against runaway loops or runaway recursion, you can cap the
number of evaluation steps per input:

```bash
# REPL with a 100000-step budget per input
cargo run -- --max-steps 100000

# Execute a script with a 50000-step budget
cargo run -- --max-steps 50000 script.expr

# Short flag form
cargo run -- -s 50000 script.expr

# Explicitly request unlimited steps (the default)
cargo run -- --max-steps -1 script.expr
```

A negative value (or zero) means **unlimited**. When the limit is
exceeded, evaluation stops and reports `Step limit exceeded (max: N)`.

Then in your Rust code:

```rust
use expr_lang::{evaluate, Value};

fn main() {
    let result = evaluate("sin(pi/2) + 3 * 2").unwrap();
    println!("{}", result); // 7.0

    // With persistent environment
    let mut env = std::collections::HashMap::new();
    let mut evaluator = expr_lang::Evaluator::new();
    expr_lang::evaluate_with_context("x = 10; fn f() = x * 2", &mut env, &mut evaluator).unwrap();
    let val = expr_lang::evaluate_with_context("f()", &mut env, &mut evaluator).unwrap();
    assert_eq!(val, expr_lang::Value::Num(20.0));
}
```

## Language Reference

### Variables

```text
x = 10
let y = 20
x + y          # 30
```

Variables are dynamically typed and globally scoped. Use `let` for clarity or direct assignment.

### Data Types

- **Numbers**: 64-bit floating point (`3.14`, `-2e5`)
- **Strings**: double-quoted, with escape sequences `\n`, `\t`, `\"`, `\\`
- **Arrays**: comma-separated values in square brackets (`[1, 2, 3]`), can be nested

### Operators

| Category | Operators |
|----------|-----------|
| Arithmetic | `+`, `-`, `*`, `/` |
| Comparison | `<`, `<=`, `>`, `>=`, `==`, `!=` |
| Logical | `&&`, `||`, `!` (prefix) |

### Control Flow

**Conditional**:

```text
if x > 0 then x
elif x == 0 then 0
else -x
```

**While Loop**:

```text
sum = 0;
i = 1;
while i <= 10 do (
    sum = sum + i;
    i = i + 1
);
sum   # 55
```

**For Loop**:

```text
sum = 0;
for i in 1..10 do sum = sum + i;
sum   # 45

# With step
for i in 1..10 step 2 do sum = sum + i;
sum   # 25

# Reverse step
for i in 10..1 step -1 do sum = sum + i;
sum   # 54
```

**Loop Control**:

```text
# break
sum = 0;
for i in 1..10 do (
    if i > 5 then break;
    sum = sum + i
);
sum   # 15

# continue
sum = 0;
for i in 1..10 do (
    if is_even(i) then continue;
    sum = sum + i
);
sum   # 25
```

### Functions

Define and call functions:

```text
fn factorial(n) = if n <= 1 then 1 else n * factorial(n - 1);
factorial(5)   # 120
```

Functions support **recursion**. They capture the environment at call time (not definition time).

### Arrays

**Indexing**:

```text
arr = [1, 2, 3, 4, 5];
arr[2]   # 3
```

**Slicing**:

```text
[1, 2, 3, 4, 5][1:4]   # [2, 3, 4]
```

### Built-in Functions

| Category | Functions |
|----------|-----------|
| Trigonometry | `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2` |
| Log/Power | `sqrt`, `exp`, `ln`, `log10`, `log2`, `log(x, base)`, `pow(base, exp)` |
| Numeric | `abs`, `floor`, `ceil`, `round`, `max`, `min`, `sign`, `hypot` |
| Integer | `factorial`, `is_even`, `is_odd` |
| Utility | `len` (string/array), `concat` (strings) |
| Conversion | `deg` (radians to degrees), `rad` (degrees to radians) |

**Constants**: `pi`, `e`

### Comments

- Single-line: `# This is a comment`
- Multi-line: `/* comment */` (supports nesting)

## Examples

### Quadratic Formula

```text
# Solve quadratic equation
a = 1; b = -3; c = 2;
discriminant = pow(b, 2) - 4 * a * c;
if discriminant >= 0 then (
    sqrt_d = sqrt(discriminant);
    x1 = (-b + sqrt_d) / (2 * a);
    x2 = (-b - sqrt_d) / (2 * a);
    [x1, x2]
) else (
    "No real roots"
)
# Output: [2, 1]
```

### Sum of Squares

```text
sum_sq = 0;
i = 1;
while i <= 5 do (
    sum_sq = sum_sq + i * i;
    i = i + 1
);
sum_sq
# Output: 55
```

### Fibonacci (Recursive)

```text
fn fib(n) = if n <= 1 then n else fib(n - 1) + fib(n - 2);
fib(10)
# Output: 55
```

### Array Processing

```text
# Sum all elements in an array
arr = [1, 2, 3, 4, 5];
sum = 0;
i = 0;
while i < len(arr) do (
    sum = sum + arr[i];
    i = i + 1
);
sum   # 15
```

## Development

### Run Tests

```bash
cargo test
```

### Run Coverage

```bash
cargo llvm-cov --html --open
```

### Project Structure

```
src/
├── lib.rs          # Library entry, exports public API
├── main.rs         # CLI entry (REPL, script execution)
├── value.rs        # Value type (Num, Str, Array)
├── ast.rs          # Abstract Syntax Tree
├── lexer.rs        # Lexical analyzer
├── parser.rs       # Recursive descent parser
├── evaluator.rs    # Evaluator and built-in functions
├── repl.rs         # REPL implementation
└── error.rs        # Error types and reporting

tests/
├── cli_tests.rs           # CLI integration tests
├── integration_tests.rs   # Public API tests
├── repl_test.rs           # REPL tests
├── script_tests.rs        # Script file tests
└── scripts/               # Test script files (*.expr)
```

### Building Documentation

```bash
cargo doc --open
```

### Contributing

Contributions are welcome! Please submit a pull request or open an issue for suggestions and bug reports.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

## Version History

### v0.3.1 (2026-09-10)

- Added `--max-steps N` / `-s N` CLI option to bound evaluation steps
  per input (`N < 0` means unlimited, `N > 0` limits to `N` steps)
- Added `Evaluator::with_max_steps`, `set_max_steps`, `reset_steps`,
  `step_count`, `max_steps`
- Added `repl_with_max_steps`; `repl()` remains as an unlimited alias
- Fixed: lexer now returns an error instead of panicking on undefined
  characters, and no longer loops forever on a lone `.`

### v0.3.0 (2026-09-09)

- Added `atan2`, `log2`, `log`, `hypot`, `factorial`, `sign`, `is_even`, `is_odd`, `deg`, `rad` functions
- Added `if-elif-else` conditional chains
- Added `for-in` loops with `step` support
- Added array slicing (`arr[1:3]`)
- Added `break` and `continue` loop control
- Added `--version` flag and CI version auto-sync
- Added comprehensive integration and CLI tests
- Added test coverage reporting with `cargo-llvm-cov`

### v0.2.x (Planned)

- Step support for for loops (moved to v0.3.0)
- Array traversal in for loops (moved to v0.3.0)

### v0.1.0 (2026-09-07)

- Initial release
- Basic arithmetic, comparisons, logic
- Variables, functions, recursion
- `if-else` conditionals, `while` loops
- Arrays and indexing
- Strings with escapes
- Comments (single-line and multi-line)
- REPL and script execution
- Comprehensive test suite

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with ❤️ in Rust
- Inspired by Python's simplicity and MATLAB's math capabilities
