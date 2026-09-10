//! Lexical analyzer for ExprLang.
//!
//! Converts source code strings into a sequence of tokens.
//! Supports numbers, strings, identifiers, operators, keywords, and comments.

/// All token types recognized by the ExprLang lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    String(String),
    Ident(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Assign,
    Comma,
    Semicolon,

    // Comparison
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,

    // Logical
    And,
    Or,
    Not,

    // Keywords
    If,
    Then,
    Else,
    Elif,
    While,
    Do,
    For,
    In,
    Step,
    Break,
    Continue,
    Fn,

    // Delimiters
    Colon,
    DotDot,
    Eof,
}

/// The lexer state.
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    /// Creates a new lexer for the given input string.
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    /// Returns the current position in the input.
    #[allow(dead_code)]
    pub fn current_position(&self) -> crate::error::Position {
        let line = self.input[..self.pos]
            .iter()
            .filter(|&&c| c == '\n')
            .count()
            + 1;
        let column = self.input[..self.pos]
            .iter()
            .rev()
            .take_while(|&&c| c != '\n')
            .count()
            + 1;
        crate::error::Position::new(line, column, self.pos)
    }

    /// Peeks at the next character without consuming it.
    fn peek_char(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    /// Skips whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Reads a numeric literal from the input.
    ///
    /// Supports integers and floating-point numbers with a decimal point.
    /// The decimal point is only considered part of the number if followed
    /// by a digit, to avoid confusion with the `..` range operator.
    ///
    /// Returns an error if no digit is consumed (e.g. a lone `.`), which
    /// previously caused the lexer to loop forever without advancing.
    fn read_number(&mut self) -> Result<f64, String> {
        let mut num_str = String::new();
        let mut has_dot = false;
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.pos += 1;
            } else if c == '.' && !has_dot {
                // Only treat '.' as decimal point if followed by a digit
                if let Some(next) = self.input.get(self.pos + 1) {
                    if next.is_ascii_digit() {
                        num_str.push(c);
                        self.pos += 1;
                        has_dot = true;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if num_str.is_empty() {
            // Guard against infinite loop on a lone '.'
            let bad = self.peek_char().unwrap_or('.');
            return Err(format!("Unexpected character: {}", bad));
        }
        num_str
            .parse()
            .map_err(|_| format!("Invalid number: {}", num_str))
    }

    /// Reads a string literal from the input.
    ///
    /// Supports escape sequences: `\n`, `\t`, `\r`, `\"`, `\\`.
    fn read_string(&mut self) -> String {
        self.pos += 1; // skip opening quote
        let mut s = String::new();
        while let Some(c) = self.peek_char() {
            if c == '"' {
                self.pos += 1;
                break;
            }
            if c == '\\' {
                self.pos += 1;
                if let Some(next) = self.peek_char() {
                    let ch = match next {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '"' => '"',
                        '\\' => '\\',
                        _ => next,
                    };
                    s.push(ch);
                    self.pos += 1;
                    continue;
                }
            }
            s.push(c);
            self.pos += 1;
        }
        s
    }

    /// Reads an identifier or keyword.
    ///
    /// Identifiers can contain alphanumeric characters and underscores.
    /// Numbers within identifiers are allowed (e.g., `atan2`).
    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        ident
    }

    /// Skips a nested multi-line comment (`/* ... */`).
    fn skip_multiline_comment(&mut self) {
        self.pos += 2;
        let mut depth = 1;
        while let Some(c) = self.peek_char() {
            if c == '/' && self.input.get(self.pos + 1) == Some(&'*') {
                // Nested comment starts
                self.pos += 2;
                depth += 1;
            } else if c == '*' && self.input.get(self.pos + 1) == Some(&'/') {
                // Nested comment ends
                self.pos += 2;
                depth -= 1;
                if depth == 0 {
                    break;
                }
            } else {
                self.pos += 1;
            }
        }
    }

    /// Returns the next token from the input.
    ///
    /// Skips whitespace and comments before returning the next token.
    pub fn next_token(&mut self) -> Result<Token, String> {
        loop {
            self.skip_whitespace();
            // Skip single-line comments starting with '#'
            if let Some('#') = self.peek_char() {
                while let Some(c) = self.peek_char() {
                    if c == '\n' {
                        break;
                    }
                    self.pos += 1;
                }
                continue;
            }
            // Skip multi-line comments
            if let (Some('/'), Some('*')) =
                (self.peek_char(), self.input.get(self.pos + 1).copied())
            {
                self.skip_multiline_comment();
                continue;
            }
            break;
        }

        match self.peek_char() {
            None => Ok(Token::Eof),
            Some(c) => {
                let next = self.input.get(self.pos + 1).copied();
                match (c, next) {
                    // Multi-character operators
                    ('<', Some('=')) => {
                        self.pos += 2;
                        Ok(Token::LessEqual)
                    }
                    ('>', Some('=')) => {
                        self.pos += 2;
                        Ok(Token::GreaterEqual)
                    }
                    ('=', Some('=')) => {
                        self.pos += 2;
                        Ok(Token::Equal)
                    }
                    ('!', Some('=')) => {
                        self.pos += 2;
                        Ok(Token::NotEqual)
                    }
                    ('&', Some('&')) => {
                        self.pos += 2;
                        Ok(Token::And)
                    }
                    ('|', Some('|')) => {
                        self.pos += 2;
                        Ok(Token::Or)
                    }
                    ('.', Some('.')) => {
                        self.pos += 2;
                        Ok(Token::DotDot)
                    }
                    // Single-character tokens
                    _ => match c {
                        '+' => {
                            self.pos += 1;
                            Ok(Token::Plus)
                        }
                        '-' => {
                            self.pos += 1;
                            Ok(Token::Minus)
                        }
                        '*' => {
                            self.pos += 1;
                            Ok(Token::Star)
                        }
                        '/' => {
                            self.pos += 1;
                            Ok(Token::Slash)
                        }
                        '(' => {
                            self.pos += 1;
                            Ok(Token::LParen)
                        }
                        ')' => {
                            self.pos += 1;
                            Ok(Token::RParen)
                        }
                        '[' => {
                            self.pos += 1;
                            Ok(Token::LBracket)
                        }
                        ']' => {
                            self.pos += 1;
                            Ok(Token::RBracket)
                        }
                        '=' => {
                            self.pos += 1;
                            Ok(Token::Assign)
                        }
                        ',' => {
                            self.pos += 1;
                            Ok(Token::Comma)
                        }
                        ';' => {
                            self.pos += 1;
                            Ok(Token::Semicolon)
                        }
                        '<' => {
                            self.pos += 1;
                            Ok(Token::Less)
                        }
                        '>' => {
                            self.pos += 1;
                            Ok(Token::Greater)
                        }
                        '!' => {
                            self.pos += 1;
                            Ok(Token::Not)
                        }
                        ':' => {
                            self.pos += 1;
                            Ok(Token::Colon)
                        }
                        '"' => {
                            let s = self.read_string();
                            Ok(Token::String(s))
                        }
                        _ if c.is_ascii_digit() || c == '.' => {
                            let num = self.read_number()?;
                            Ok(Token::Number(num))
                        }
                        _ if c.is_alphabetic() || c == '_' => {
                            let id = self.read_ident();
                            match id.as_str() {
                                "if" => Ok(Token::If),
                                "then" => Ok(Token::Then),
                                "else" => Ok(Token::Else),
                                "elif" => Ok(Token::Elif),
                                "while" => Ok(Token::While),
                                "do" => Ok(Token::Do),
                                "for" => Ok(Token::For),
                                "in" => Ok(Token::In),
                                "step" => Ok(Token::Step),
                                "break" => Ok(Token::Break),
                                "continue" => Ok(Token::Continue),
                                "fn" => Ok(Token::Fn),
                                _ => Ok(Token::Ident(id)),
                            }
                        }
                        _ => Err(format!("Unexpected character: {}", c)),
                    },
                }
            }
        }
    }
}
