use crate::lexer::{Lexer, Token};
use crate::ast::{Expr, Op, UnaryOp};

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Parser { lexer, current }
    }

    fn next_token(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let mut seq = Vec::new();
        loop {
            match self.current {
                Token::EOF => break,
                _ => {
                    let expr = self.parse_expr()?;
                    seq.push(expr);
                    if self.current == Token::Semicolon {
                        self.next_token();
                    } else {
                        break;
                    }
                }
            }
        }
        if seq.len() == 1 {
            Ok(seq.remove(0))
        } else {
            Ok(Expr::Sequence(seq))
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match &self.current {
            Token::If => return self.parse_if(),
            Token::While => return self.parse_while(),
            Token::Fn => return self.parse_function_def(),
            Token::Ident(name) if name == "let" => {
                self.next_token();
                if let Token::Ident(var_name) = &self.current {
                    let var_name = var_name.clone();
                    self.next_token();
                    if self.current != Token::Assign {
                        return Err("Expected '=' after let".to_string());
                    }
                    self.next_token();
                    let right = self.parse_expr()?;
                    return Ok(Expr::Assign { name: var_name, expr: Box::new(right) });
                } else {
                    return Err("Expected variable name after let".to_string());
                }
            }
            _ => {}
        }

        let left = self.parse_or()?;
        if self.current == Token::Assign {
            if let Expr::Variable(name) = left {
                self.next_token();
                let right = self.parse_expr()?;
                Ok(Expr::Assign { name, expr: Box::new(right) })
            } else {
                Err("Assignment target must be a variable".to_string())
            }
        } else {
            Ok(left)
        }
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.current == Token::Or {
            self.next_token();
            let right = self.parse_and()?;
            left = Expr::Binary { op: Op::Or, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        while self.current == Token::And {
            self.next_token();
            let right = self.parse_comparison()?;
            left = Expr::Binary { op: Op::And, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let left = self.parse_add_sub()?;
        match self.current {
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual |
            Token::Equal | Token::NotEqual => {
                let op = match self.current {
                    Token::Less => Op::Less,
                    Token::LessEqual => Op::LessEqual,
                    Token::Greater => Op::Greater,
                    Token::GreaterEqual => Op::GreaterEqual,
                    Token::Equal => Op::Equal,
                    Token::NotEqual => Op::NotEqual,
                    _ => unreachable!(),
                };
                self.next_token();
                let right = self.parse_comparison()?;
                Ok(Expr::Binary { op, left: Box::new(left), right: Box::new(right) })
            }
            _ => Ok(left),
        }
    }

    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul_div()?;
        while let Token::Plus | Token::Minus = self.current {
            let op = match self.current {
                Token::Plus => Op::Add,
                Token::Minus => Op::Sub,
                _ => unreachable!(),
            };
            self.next_token();
            let right = self.parse_mul_div()?;
            left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        while let Token::Star | Token::Slash = self.current {
            let op = match self.current {
                Token::Star => Op::Mul,
                Token::Slash => Op::Div,
                _ => unreachable!(),
            };
            self.next_token();
            let right = self.parse_unary()?;
            left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.current {
            Token::Minus => {
                self.next_token();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::Neg, expr: Box::new(expr) })
            }
            Token::Not => {
                self.next_token();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::Not, expr: Box::new(expr) })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.current {
                Token::LBracket => {
                    self.next_token();
                    let index = self.parse_expr()?;
                    if self.current != Token::RBracket {
                        return Err("Expected ']'".to_string());
                    }
                    self.next_token();
                    expr = Expr::Index {
                        array: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current {
            Token::Number(n) => {
                let num = *n;
                self.next_token();
                Ok(Expr::Number(num))
            }
            Token::String(s) => {
                let s = s.clone();
                self.next_token();
                Ok(Expr::String(s))
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.next_token();
                if self.current == Token::LParen {
                    self.next_token();
                    let mut args = Vec::new();
                    if self.current != Token::RParen {
                        loop {
                            let arg = self.parse_expr()?;
                            args.push(arg);
                            if self.current == Token::Comma {
                                self.next_token();
                            } else {
                                break;
                            }
                        }
                    }
                    if self.current != Token::RParen {
                        return Err("Expected ')'".to_string());
                    }
                    self.next_token();
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Variable(name))
                }
            }
            Token::LParen => {
                self.next_token();
                let mut seq = Vec::new();
                let first = self.parse_expr()?;
                seq.push(first);
                while self.current == Token::Semicolon {
                    self.next_token();
                    if self.current == Token::RParen {
                        break;
                    }
                    let next = self.parse_expr()?;
                    seq.push(next);
                }
                if self.current != Token::RParen {
                    return Err("Expected ')'".to_string());
                }
                self.next_token();
                if seq.len() == 1 {
                    Ok(seq.remove(0))
                } else {
                    Ok(Expr::Sequence(seq))
                }
            }
            Token::LBracket => {
                self.next_token();
                let mut elems = Vec::new();
                if self.current != Token::RBracket {
                    loop {
                        let elem = self.parse_expr()?;
                        elems.push(elem);
                        if self.current == Token::Comma {
                            self.next_token();
                        } else {
                            break;
                        }
                    }
                }
                if self.current != Token::RBracket {
                    return Err("Expected ']'".to_string());
                }
                self.next_token();
                Ok(Expr::Array(elems))
            }
            _ => Err(format!("Unexpected token: {:?}", self.current)),
        }
    }

    fn parse_if(&mut self) -> Result<Expr, String> {
        self.next_token();
        let cond = Box::new(self.parse_expr()?);
        if self.current != Token::Then {
            return Err("Expected 'then'".to_string());
        }
        self.next_token();
        let then_expr = Box::new(self.parse_expr()?);
        if self.current != Token::Else {
            return Err("Expected 'else'".to_string());
        }
        self.next_token();
        let else_expr = Box::new(self.parse_expr()?);
        Ok(Expr::If { cond, then: then_expr, else_branch: else_expr })
    }

    fn parse_while(&mut self) -> Result<Expr, String> {
        self.next_token();
        let cond = Box::new(self.parse_expr()?);
        if self.current != Token::Do {
            return Err("Expected 'do'".to_string());
        }
        self.next_token();
        let body = Box::new(self.parse_expr()?);
        Ok(Expr::While { cond, body })
    }

    fn parse_function_def(&mut self) -> Result<Expr, String> {
        self.next_token();
        let name = match &self.current {
            Token::Ident(n) => n.clone(),
            _ => return Err("Expected function name".to_string()),
        };
        self.next_token();
        if self.current != Token::LParen {
            return Err("Expected '('".to_string());
        }
        self.next_token();
        let mut params = Vec::new();
        if self.current != Token::RParen {
            loop {
                if let Token::Ident(p) = &self.current {
                    params.push(p.clone());
                    self.next_token();
                    if self.current == Token::Comma {
                        self.next_token();
                    } else if self.current == Token::RParen {
                        break;
                    } else {
                        return Err("Expected ',' or ')'".to_string());
                    }
                } else {
                    return Err("Expected parameter name".to_string());
                }
            }
        }
        self.next_token();
        if self.current != Token::Assign {
            return Err("Expected '='".to_string());
        }
        self.next_token();
        let body = Box::new(self.parse_expr()?);
        Ok(Expr::FunctionDef { name, params, body })
    }
}
