use crate::ast::Expr;
use crate::value::Value;
use std::collections::HashMap;
use std::f64::consts;

pub struct Evaluator {
    functions: HashMap<String, (Vec<String>, Expr)>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            functions: HashMap::new(),
        }
    }

    pub fn eval(&mut self, expr: &Expr, env: &mut HashMap<String, Value>) -> Result<Value, String> {
        self.eval_internal(expr, env)
    }

    fn eval_internal(
        &mut self,
        expr: &Expr,
        env: &mut HashMap<String, Value>,
    ) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Num(*n)),
            Expr::String(s) => Ok(Value::Str(s.clone())),
            Expr::Array(elems) => {
                let mut vals = Vec::new();
                for e in elems {
                    vals.push(self.eval_internal(e, env)?);
                }
                Ok(Value::Array(vals))
            }
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
            Expr::Variable(name) => env
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),
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
                    crate::ast::Op::Less => Ok(Value::Num(if l < r { 1.0 } else { 0.0 })),
                    crate::ast::Op::LessEqual => Ok(Value::Num(if l <= r { 1.0 } else { 0.0 })),
                    crate::ast::Op::Greater => Ok(Value::Num(if l > r { 1.0 } else { 0.0 })),
                    crate::ast::Op::GreaterEqual => Ok(Value::Num(if l >= r { 1.0 } else { 0.0 })),
                    crate::ast::Op::Equal => Ok(Value::Num(if l == r { 1.0 } else { 0.0 })),
                    crate::ast::Op::NotEqual => Ok(Value::Num(if l != r { 1.0 } else { 0.0 })),
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
                    let mut local_env = env.clone();
                    for (p, v) in params.iter().zip(arg_vals) {
                        local_env.insert(p.clone(), v);
                    }
                    self.eval_internal(&body, &mut local_env)
                } else {
                    Err(format!("Unknown function: {}", name))
                }
            }
            Expr::Assign { name, expr } => {
                let val = self.eval_internal(expr, env)?;
                env.insert(name.clone(), val.clone());
                Ok(val)
            }
            Expr::Sequence(seq) => {
                let mut last = Value::Num(0.0);
                for e in seq {
                    last = self.eval_internal(e, env)?;
                }
                Ok(last)
            }
            Expr::If {
                cond,
                then,
                else_branch,
            } => {
                let cond_val = self.eval_internal(cond, env)?;
                if Self::as_bool(&cond_val)? {
                    self.eval_internal(then, env)
                } else {
                    self.eval_internal(else_branch, env)
                }
            }
            Expr::While { cond, body } => {
                let mut result = Value::Num(0.0);
                while Self::as_bool(&self.eval_internal(cond, env)?)? {
                    result = self.eval_internal(body, env)?;
                }
                Ok(result)
            }
            Expr::FunctionDef { name, params, body } => {
                self.functions
                    .insert(name.clone(), (params.clone(), *body.clone()));
                Ok(Value::Num(0.0))
            }
        }
    }

    fn as_bool(val: &Value) -> Result<bool, String> {
        match val {
            Value::Num(n) => Ok(*n != 0.0),
            Value::Str(s) => Ok(!s.is_empty()),
            Value::Array(a) => Ok(!a.is_empty()),
        }
    }

    fn get_builtin(name: &str) -> Option<BuiltinFunc> {
        use BuiltinFunc::*;
        match name {
            "sin" => Some(Sin),
            "cos" => Some(Cos),
            "tan" => Some(Tan),
            "asin" => Some(Asin),
            "acos" => Some(Acos),
            "atan" => Some(Atan),
            "sqrt" => Some(Sqrt),
            "exp" => Some(Exp),
            "ln" => Some(Ln),
            "log10" => Some(Log10),
            "abs" => Some(Abs),
            "pow" => Some(Pow),
            "max" => Some(Max),
            "min" => Some(Min),
            "floor" => Some(Floor),
            "ceil" => Some(Ceil),
            "round" => Some(Round),
            "len" => Some(Len),
            "concat" => Some(Concat),
            _ => None,
        }
    }

    fn eval_builtin(
        &mut self,
        func: BuiltinFunc,
        args: &[Expr],
        env: &mut HashMap<String, Value>,
    ) -> Result<Value, String> {
        use BuiltinFunc::*;
        match func {
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
            Abs => self.eval_unary_num(args, env, |x| x.abs()),
            Floor => self.eval_unary_num(args, env, |x| x.floor()),
            Ceil => self.eval_unary_num(args, env, |x| x.ceil()),
            Round => self.eval_unary_num(args, env, |x| x.round()),
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
        }
    }

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

enum BuiltinFunc {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sqrt,
    Exp,
    Ln,
    Log10,
    Abs,
    Max,
    Min,
    Floor,
    Ceil,
    Round,
    Len,
    Concat,
    Pow,
}

// ========== 公共求值函数 ==========
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
