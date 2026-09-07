# ExprLang

**ExprLang** is a lightweight, Rust-based **mathematical expression language** designed for scientific computing, formula evaluation, and scripting. It combines the simplicity of a calculator with the power of a scripting language.

[![Crates.io](https://img.shields.io/crates/v/expr_lang.svg)](https://crates.io/crates/expr_lang)
[![Documentation](https://docs.rs/expr_lang/badge.svg)](https://docs.rs/expr_lang)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/Zcytxcbyz/ExprLang/actions/workflows/ci.yml/badge.svg)](https://github.com/Zcytxcbyz/ExprLang/actions/workflows/ci.yml)

## Features

- **Arithmetic**: `+`, `-`, `*`, `/`, parentheses, unary minus
- **Comparisons**: `<`, `<=`, `>`, `>=`, `==`, `!=`
- **Logic**: `&&`, `||`, `!` (non-zero values are truthy)
- **Variables**: dynamic typing, global scope, `let` or direct assignment
- **Data types**: numbers (`f64`), strings (with escapes), arrays (nested)
- **Control flow**: `if cond then expr else expr`, `while cond do expr`
- **Functions**: `fn name(params) = body`, supports recursion, closure capture
- **Built-in functions**: trigonometric, logarithmic, power, `max`, `min`, `len`, `concat`, etc.
- **Comments**: single-line `#`, multi-line `/* */` (nested)
- **REPL**: interactive session for quick calculations
- **Script execution**: run `.expr` files from the command line
- **Comprehensive test suite**: unit + integration + script tests
- **Fast compilation**: builds in under 0.1 seconds

## Installation

### From Crates.io

```bash
cargo add expr_lang
```

### From Source

```bash
git clone https://github.com/yourusername/exprlang
cd exprlang
cargo build --release
```

## Usage

### REPL (Interactive)

Start the interactive REPL:

```bash
cargo run
```

Example session:
```text
ExprLang v0.1.0 (Rust Math Expression Language)
Supported: arithmetic, comparisons, logic (&&, ||, !),
strings, arrays, indexing, functions, loops, conditions.

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

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
expr_lang = "0.1.0"
```

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
if x > 0 then x else -x
```

**Loop**:
```text
sum = 0;
i = 1;
while i <= 10 do (
    sum = sum + i;
    i = i + 1
);
sum   # 55
```

### Functions

Define and call functions:

```text
fn factorial(n) = if n <= 1 then 1 else n * factorial(n - 1)
factorial(5)   # 120
```

Functions are **first-class** (can be stored? Not yet) and support **recursion**. They capture the environment at call time (not definition time).

### Built-in Functions

| Category | Functions |
|----------|-----------|
| Trigonometry | `sin`, `cos`, `tan`, `asin`, `acos`, `atan` |
| Log/Power | `sqrt`, `exp`, `ln`, `log10`, `pow(base, exp)` |
| Numeric | `abs`, `floor`, `ceil`, `round`, `max`, `min` |
| Utility | `len` (string/array), `concat` (strings) |

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

## Development

### Run Tests

```bash
cargo test
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
└── repl.rs         # REPL implementation

tests/
├── integration_tests.rs  # Public API tests
├── script_tests.rs       # Script file tests
└── scripts/              # Test script files (*.expr)
```

### Building Documentation

```bash
cargo doc --open
```

### Contributing

Contributions are welcome! Please submit a pull request or open an issue for suggestions and bug reports.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with ❤️ in Rust
- Inspired by Python's simplicity and MATLAB's math capabilities
