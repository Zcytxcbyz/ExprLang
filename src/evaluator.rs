//! Evaluator for ExprLang expressions.
//!
//! The evaluator traverses the AST and computes values using a
//! dynamic environment for variables and function definitions.

use crate::ast::Expr;
use crate::value::Value;
use std::collections::HashMap;
use std::f64::consts;

/// The evaluator state, holding user-defined functions.
#[derive(Default)]
pub struct Evaluator {
    functions: HashMap<String, (Vec<String>, Expr)>,
}

impl Evaluator {
    /// Creates a new evaluator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluates an expression in the given environment.
    pub fn eval(&mut self, expr: &Expr, env: &mut HashMap<String, Value>) -> Result<Value, String> {
        self.eval_internal(expr, env)
    }

    /// Internal recursive evaluator.
    fn eval_internal(
        &mut self,
        expr: &Expr,
        env: &mut HashMap<String, Value>,
    ) -> Result<Value, String> {
        match expr {
            // Literals
            Expr::Number(n) => Ok(Value::Num(*n)),
            Expr::String(s) => Ok(Value::Str(s.clone())),
            Expr::Array(elems) => {
                let mut vals = Vec::new();
                for e in elems {
                    vals.push(self.eval_internal(e, env)?);
                }
                Ok(Value::Array(vals))
            }

            // Array indexing
            Expr::Index { array, index } => {
                let arr = self.eval_internal(array, env)?;
                let idx = self.eval_internal(index, env)?;
                match (arr, idx) {
                    (Value::Array(a), Value::Num(i)) => {
                        let i = i as usize;
                        if i < a.len() {
                            Ok(a[i].clone())
                        } else {
                            Err(format!("Index out of bounds: {}", i))
                        }
                    }
                    _ => Err("Index requires an array and a numeric index".to_string()),
                }
            }

            // Array slicing
            Expr::Slice { array, start, end } => {
                let arr = self.eval_internal(array, env)?;
                match arr {
                    Value::Array(a) => {
                        let start_idx = if let Some(s) = start {
                            match self.eval_internal(s, env)? {
                                Value::Num(n) => n as usize,
                                _ => return Err("Slice start must be a number".to_string()),
                            }
                        } else {
                            0
                        };
                        let end_idx = if let Some(e) = end {
                            match self.eval_internal(e, env)? {
                                Value::Num(n) => n as usize,
                                _ => return Err("Slice end must be a number".to_string()),
                            }
                        } else {
                            a.len()
                        };
                        if start_idx > a.len() || end_idx > a.len() || start_idx > end_idx {
                            return Err("Invalid slice bounds".to_string());
                        }
                        Ok(Value::Array(a[start_idx..end_idx].to_vec()))
                    }
                    _ => Err("Slice requires an array".to_string()),
                }
            }

            // Variable reference
            Expr::Variable(name) => env
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),

            // Binary operations
            Expr::Binary { op, left, right } => {
                let l = self.eval_internal(left, env)?;
                let r = self.eval_internal(right, env)?;
                match op {
                    crate::ast::Op::Add => match (l, r) {
                        (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a + b)),
                        (Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + &b)),
                        _ => Err("+ requires two numbers or two strings".to_string()),
                    },
                    crate::ast::Op::Sub => {
                        if let (Value::Num(a), Value::Num(b)) = (l, r) {
                            Ok(Value::Num(a - b))
                        } else {
                            Err("- requires numbers".to_string())
                        }
                    }
                    crate::ast::Op::Mul => {
                        if let (Value::Num(a), Value::Num(b)) = (l, r) {
                            Ok(Value::Num(a * b))
                        } else {
                            Err("* requires numbers".to_string())
                        }
                    }
                    crate::ast::Op::Div => {
                        if let (Value::Num(a), Value::Num(b)) = (l, r) {
                            if b == 0.0 {
                                Err("Division by zero".to_string())
                            } else {
                                Ok(Value::Num(a / b))
                            }
                        } else {
                            Err("/ requires numbers".to_string())
                        }
                    }
                    // Comparison operators return 1.0 for true, 0.0 for false
                    crate::ast::Op::Less => Ok(Value::Num(if l < r { 1.0 } else { 0.0 })),
                    crate::ast::Op::LessEqual => Ok(Value::Num(if l <= r { 1.0 } else { 0.0 })),
                    crate::ast::Op::Greater => Ok(Value::Num(if l > r { 1.0 } else { 0.0 })),
                    crate::ast::Op::GreaterEqual => Ok(Value::Num(if l >= r { 1.0 } else { 0.0 })),
                    crate::ast::Op::Equal => Ok(Value::Num(if l == r { 1.0 } else { 0.0 })),
                    crate::ast::Op::NotEqual => Ok(Value::Num(if l != r { 1.0 } else { 0.0 })),
                    // Logical operators
                    crate::ast::Op::And => {
                        let a = Self::as_bool(&l)?;
                        let b = Self::as_bool(&r)?;
                        Ok(Value::Num(if a && b { 1.0 } else { 0.0 }))
                    }
                    crate::ast::Op::Or => {
                        let a = Self::as_bool(&l)?;
                        let b = Self::as_bool(&r)?;
                        Ok(Value::Num(if a || b { 1.0 } else { 0.0 }))
                    }
                }
            }

            // Unary operations
            Expr::Unary { op, expr } => {
                let val = self.eval_internal(expr, env)?;
                match op {
                    crate::ast::UnaryOp::Neg => {
                        if let Value::Num(n) = val {
                            Ok(Value::Num(-n))
                        } else {
                            Err("Negation requires a number".to_string())
                        }
                    }
                    crate::ast::UnaryOp::Not => {
                        let b = Self::as_bool(&val)?;
                        Ok(Value::Num(if !b { 1.0 } else { 0.0 }))
                    }
                }
            }

            // Control flow: break and continue
            Expr::Break => return Err("__break__".to_string()),
            Expr::Continue => return Err("__continue__".to_string()),

            // Function calls (built-in or user-defined)
            Expr::Call { name, args } => {
                if let Some(builtin) = Self::get_builtin(name) {
                    return self.eval_builtin(builtin, args, env);
                }
                if let Some((params, body)) = self.functions.get(name).cloned() {
                    let mut arg_vals = Vec::new();
                    for arg in args {
                        arg_vals.push(self.eval_internal(arg, env)?);
                    }
                    if arg_vals.len() != params.len() {
                        return Err(format!(
                            "Function {} expects {} arguments, got {}",
                            name,
                            params.len(),
                            arg_vals.len()
                        ));
                    }
                    // Create a new environment with parameters bound
                    let mut local_env = env.clone();
                    for (p, v) in params.iter().zip(arg_vals) {
                        local_env.insert(p.clone(), v);
                    }
                    self.eval_internal(&body, &mut local_env)
                } else {
                    Err(format!("Unknown function: {}", name))
                }
            }

            // Variable assignment
            Expr::Assign { name, expr } => {
                let val = self.eval_internal(expr, env)?;
                env.insert(name.clone(), val.clone());
                Ok(val)
            }

            // Expression sequence
            Expr::Sequence(seq) => {
                let mut last = Value::Num(0.0);
                for e in seq {
                    last = self.eval_internal(e, env)?;
                }
                Ok(last)
            }

            // Conditional expression
            Expr::If { branches, else_branch } => {
                for (cond, then_expr) in branches {
                    let cond_val = self.eval_internal(cond, env)?;
                    if Self::as_bool(&cond_val)? {
                        return self.eval_internal(then_expr, env);
                    }
                }
                if let Some(else_expr) = else_branch {
                    self.eval_internal(else_expr, env)
                } else {
                    Ok(Value::Num(0.0))
                }
            }

            // While loop
            Expr::While { cond, body } => {
                let mut result = Value::Num(0.0);
                while Self::as_bool(&self.eval_internal(cond, env)?)? {
                    match self.eval_internal(body, env) {
                        Ok(v) => result = v,
                        Err(e) if e == "__break__" => break,
                        Err(e) if e == "__continue__" => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(result)
            }

            // For loop
            Expr::For {
                var,
                start,
                end,
                step,
                body,
            } => {
                let start_val = self.eval_internal(start, env)?;
                let end_val = self.eval_internal(end, env)?;
                let step_val = if let Some(s) = step {
                    match self.eval_internal(s, env)? {
                        Value::Num(n) => n,
                        _ => return Err("Step must be a number".to_string()),
                    }
                } else {
                    1.0
                };

                let (start_num, end_num) = match (start_val, end_val) {
                    (Value::Num(a), Value::Num(b)) => (a, b),
                    _ => return Err("Range bounds must be numbers".to_string()),
                };

                let mut result = Value::Num(0.0);
                let mut i = start_num;
                if step_val > 0.0 {
                    while i < end_num {
                        env.insert(var.clone(), Value::Num(i));
                        match self.eval_internal(body, env) {
                            Ok(v) => result = v,
                            Err(e) if e == "__break__" => break,
                            Err(e) if e == "__continue__" => {
                                i += step_val;
                                continue;
                            }
                            Err(e) => return Err(e),
                        }
                        i += step_val;
                    }
                } else if step_val < 0.0 {
                    while i > end_num {
                        env.insert(var.clone(), Value::Num(i));
                        match self.eval_internal(body, env) {
                            Ok(v) => result = v,
                            Err(e) if e == "__break__" => break,
                            Err(e) if e == "__continue__" => {
                                i += step_val;
                                continue;
                            }
                            Err(e) => return Err(e),
                        }
                        i += step_val;
                    }
                } else {
                    return Err("Step cannot be zero".to_string());
                }
                Ok(result)
            }

            // Function definition
            Expr::FunctionDef { name, params, body } => {
                self.functions
                    .insert(name.clone(), (params.clone(), *body.clone()));
                Ok(Value::Num(0.0))
            }
        }
    }

    /// Converts a value to a boolean.
    ///
    /// Numbers: non-zero is true, zero is false.
    /// Strings: non-empty is true, empty is false.
    /// Arrays: non-empty is true, empty is false.
    fn as_bool(val: &Value) -> Result<bool, String> {
        match val {
            Value::Num(n) => Ok(*n != 0.0),
            Value::Str(s) => Ok(!s.is_empty()),
            Value::Array(a) => Ok(!a.is_empty()),
        }
    }

    /// Returns the built-in function for the given name.
    fn get_builtin(name: &str) -> Option<BuiltinFunc> {
        use BuiltinFunc::*;
        match name {
            // Trigonometric
            "sin" => Some(Sin),
            "cos" => Some(Cos),
            "tan" => Some(Tan),
            "asin" => Some(Asin),
            "acos" => Some(Acos),
            "atan" => Some(Atan),
            "atan2" => Some(Atan2),
            // Power and log
            "sqrt" => Some(Sqrt),
            "exp" => Some(Exp),
            "ln" => Some(Ln),
            "log10" => Some(Log10),
            "log2" => Some(Log2),
            "log" => Some(Log),
            "hypot" => Some(Hypot),
            "pow" => Some(Pow),
            // Numeric
            "abs" => Some(Abs),
            "max" => Some(Max),
            "min" => Some(Min),
            "floor" => Some(Floor),
            "ceil" => Some(Ceil),
            "round" => Some(Round),
            // Utility
            "len" => Some(Len),
            "concat" => Some(Concat),
            "factorial" => Some(Factorial),
            "sign" => Some(Sign),
            "is_even" => Some(IsEven),
            "is_odd" => Some(IsOdd),
            "deg" => Some(Deg),
            "rad" => Some(Rad),
            _ => None,
        }
    }

    /// Evaluates a built-in function call.
    fn eval_builtin(
        &mut self,
        func: BuiltinFunc,
        args: &[Expr],
        env: &mut HashMap<String, Value>,
    ) -> Result<Value, String> {
        use BuiltinFunc::*;
        match func {
            // Unary numeric functions
            Sin => self.eval_unary_num(args, env, |x| x.sin()),
            Cos => self.eval_unary_num(args, env, |x| x.cos()),
            Tan => self.eval_unary_num(args, env, |x| x.tan()),
            Asin => self.eval_unary_num(args, env, |x| x.asin()),
            Acos => self.eval_unary_num(args, env, |x| x.acos()),
            Atan => self.eval_unary_num(args, env, |x| x.atan()),
            Sqrt => self.eval_unary_num(args, env, |x| x.sqrt()),
            Exp => self.eval_unary_num(args, env, |x| x.exp()),
            Ln => self.eval_unary_num(args, env, |x| x.ln()),
            Log10 => self.eval_unary_num(args, env, |x| x.log10()),
            Log2 => self.eval_unary_num(args, env, |x| x.log2()),
            Abs => self.eval_unary_num(args, env, |x| x.abs()),
            Floor => self.eval_unary_num(args, env, |x| x.floor()),
            Ceil => self.eval_unary_num(args, env, |x| x.ceil()),
            Round => self.eval_unary_num(args, env, |x| x.round()),

            // Multi-argument functions
            Atan2 => {
                if args.len() != 2 {
                    return Err("atan2 takes 2 arguments".to_string());
                }
                let y = self.eval_internal(&args[0], env)?;
                let x = self.eval_internal(&args[1], env)?;
                match (y, x) {
                    (Value::Num(yv), Value::Num(xv)) => Ok(Value::Num(yv.atan2(xv))),
                    _ => Err("atan2 requires numeric arguments".to_string()),
                }
            }
            Log => {
                if args.len() != 2 {
                    return Err("log takes 2 arguments (x, base)".to_string());
                }
                let x = self.eval_internal(&args[0], env)?;
                let base = self.eval_internal(&args[1], env)?;
                match (x, base) {
                    (Value::Num(xv), Value::Num(bv)) => Ok(Value::Num(xv.log(bv))),
                    _ => Err("log requires numeric arguments".to_string()),
                }
            }
            Hypot => {
                if args.len() != 2 {
                    return Err("hypot takes 2 arguments".to_string());
                }
                let x = self.eval_internal(&args[0], env)?;
                let y = self.eval_internal(&args[1], env)?;
                match (x, y) {
                    (Value::Num(xv), Value::Num(yv)) => Ok(Value::Num(xv.hypot(yv))),
                    _ => Err("hypot requires numeric arguments".to_string()),
                }
            }
            Pow => {
                if args.len() != 2 {
                    return Err("pow takes 2 arguments".to_string());
                }
                let base = self.eval_internal(&args[0], env)?;
                let exp = self.eval_internal(&args[1], env)?;
                if let (Value::Num(b), Value::Num(e)) = (base, exp) {
                    Ok(Value::Num(b.powf(e)))
                } else {
                    Err("pow requires numeric arguments".to_string())
                }
            }
            Max => {
                if args.len() < 2 {
                    return Err("max requires at least 2 arguments".to_string());
                }
                let mut values = Vec::new();
                for arg in args {
                    values.push(self.eval_internal(arg, env)?);
                }
                let mut max_val = values[0].clone();
                for v in &values[1..] {
                    if *v > max_val {
                        max_val = v.clone();
                    }
                }
                Ok(max_val)
            }
            Min => {
                if args.len() < 2 {
                    return Err("min requires at least 2 arguments".to_string());
                }
                let mut values = Vec::new();
                for arg in args {
                    values.push(self.eval_internal(arg, env)?);
                }
                let mut min_val = values[0].clone();
                for v in &values[1..] {
                    if *v < min_val {
                        min_val = v.clone();
                    }
                }
                Ok(min_val)
            }
            Len => {
                if args.len() != 1 {
                    return Err("len takes 1 argument".to_string());
                }
                let val = self.eval_internal(&args[0], env)?;
                match val {
                    Value::Str(s) => Ok(Value::Num(s.len() as f64)),
                    Value::Array(a) => Ok(Value::Num(a.len() as f64)),
                    _ => Err("len requires a string or array".to_string()),
                }
            }
            Concat => {
                let mut result = String::new();
                for arg in args {
                    let val = self.eval_internal(arg, env)?;
                    if let Value::Str(s) = val {
                        result.push_str(&s);
                    } else {
                        return Err("concat requires string arguments".to_string());
                    }
                }
                Ok(Value::Str(result))
            }
            Factorial => {
                if args.len() != 1 {
                    return Err("factorial takes 1 argument".to_string());
                }
                let n = self.eval_internal(&args[0], env)?;
                match n {
                    Value::Num(nv) => {
                        if nv < 0.0 || nv.fract() != 0.0 {
                            return Err("factorial requires a non-negative integer".to_string());
                        }
                        let n = nv as u64;
                        if n > 20 {
                            return Err("factorial too large (max 20)".to_string());
                        }
                        let result = (1..=n).product::<u64>();
                        Ok(Value::Num(result as f64))
                    }
                    _ => Err("factorial requires a number".to_string()),
                }
            }
            Sign => {
                if args.len() != 1 {
                    return Err("sign takes 1 argument".to_string());
                }
                let val = self.eval_internal(&args[0], env)?;
                match val {
                    Value::Num(n) => {
                        if n > 0.0 {
                            Ok(Value::Num(1.0))
                        } else if n < 0.0 {
                            Ok(Value::Num(-1.0))
                        } else {
                            Ok(Value::Num(0.0))
                        }
                    }
                    _ => Err("sign requires a number".to_string()),
                }
            }
            IsEven => {
                if args.len() != 1 {
                    return Err("is_even takes 1 argument".to_string());
                }
                let val = self.eval_internal(&args[0], env)?;
                match val {
                    Value::Num(n) => {
                        if n.fract() == 0.0 {
                            Ok(Value::Num(if (n as i64) % 2 == 0 { 1.0 } else { 0.0 }))
                        } else {
                            Err("is_even requires an integer".to_string())
                        }
                    }
                    _ => Err("is_even requires a number".to_string()),
                }
            }
            IsOdd => {
                if args.len() != 1 {
                    return Err("is_odd takes 1 argument".to_string());
                }
                let val = self.eval_internal(&args[0], env)?;
                match val {
                    Value::Num(n) => {
                        if n.fract() == 0.0 {
                            Ok(Value::Num(if (n as i64) % 2 != 0 { 1.0 } else { 0.0 }))
                        } else {
                            Err("is_odd requires an integer".to_string())
                        }
                    }
                    _ => Err("is_odd requires a number".to_string()),
                }
            }
            Deg => {
                if args.len() != 1 {
                    return Err("deg takes 1 argument".to_string());
                }
                let val = self.eval_internal(&args[0], env)?;
                match val {
                    Value::Num(n) => Ok(Value::Num(n * 180.0 / consts::PI)),
                    _ => Err("deg requires a number".to_string()),
                }
            }
            Rad => {
                if args.len() != 1 {
                    return Err("rad takes 1 argument".to_string());
                }
                let val = self.eval_internal(&args[0], env)?;
                match val {
                    Value::Num(n) => Ok(Value::Num(n * consts::PI / 180.0)),
                    _ => Err("rad requires a number".to_string()),
                }
            }
        }
    }

    /// Helper for unary numeric built-in functions.
    fn eval_unary_num<F>(
        &mut self,
        args: &[Expr],
        env: &mut HashMap<String, Value>,
        f: F,
    ) -> Result<Value, String>
    where
        F: Fn(f64) -> f64,
    {
        if args.len() != 1 {
            return Err(format!("Function takes 1 argument, got {}", args.len()));
        }
        let val = self.eval_internal(&args[0], env)?;
        if let Value::Num(n) = val {
            Ok(Value::Num(f(n)))
        } else {
            Err("Argument must be a number".to_string())
        }
    }
}

/// All built-in functions supported by the evaluator.
enum BuiltinFunc {
    // Trigonometric
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Atan2,
    // Power and log
    Sqrt,
    Exp,
    Ln,
    Log10,
    Log2,
    Log,
    Hypot,
    Pow,
    // Numeric
    Abs,
    Max,
    Min,
    Floor,
    Ceil,
    Round,
    // Utility
    Len,
    Concat,
    Factorial,
    Sign,
    IsEven,
    IsOdd,
    Deg,
    Rad,
}

/// Evaluates an expression string in a fresh environment.
pub fn evaluate(expr: &str) -> Result<Value, String> {
    let lexer = crate::lexer::Lexer::new(expr);
    let mut parser = crate::parser::Parser::new(lexer);
    let ast = parser.parse()?;
    let mut evaluator = Evaluator::new();
    let mut env = HashMap::new();
    env.insert("pi".to_string(), Value::Num(consts::PI));
    env.insert("e".to_string(), Value::Num(consts::E));
    evaluator.eval(&ast, &mut env)
}

/// Evaluates an expression with a persistent environment.
pub fn evaluate_with_env(expr: &str, env: &mut HashMap<String, Value>) -> Result<Value, String> {
    let lexer = crate::lexer::Lexer::new(expr);
    let mut parser = crate::parser::Parser::new(lexer);
    let ast = parser.parse()?;
    let mut evaluator = Evaluator::new();
    env.entry("pi".to_string())
        .or_insert(Value::Num(consts::PI));
    env.entry("e".to_string()).or_insert(Value::Num(consts::E));
    evaluator.eval(&ast, env)
}

/// Evaluates an expression with a persistent evaluator and environment.
pub fn evaluate_with_context(
    expr: &str,
    env: &mut HashMap<String, Value>,
    evaluator: &mut Evaluator,
) -> Result<Value, String> {
    let lexer = crate::lexer::Lexer::new(expr);
    let mut parser = crate::parser::Parser::new(lexer);
    let ast = parser.parse()?;
    evaluator.eval(&ast, env)
}
