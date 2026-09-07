#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Ident(String),
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
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
    Not,
    If,
    Then,
    Else,
    While,
    Do,
    Fn,
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        num_str.parse().unwrap_or(0.0)
    }

    fn read_string(&mut self) -> String {
        self.pos += 1; // skip "
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

    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_alphabetic() || c == '_' {
                ident.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        ident
    }

    fn skip_multiline_comment(&mut self) {
        self.pos += 2;
        let mut depth = 1;
        while let Some(c) = self.peek_char() {
            if c == '/' && self.input.get(self.pos + 1) == Some(&'*') {
                self.pos += 2;
                depth += 1;
            } else if c == '*' && self.input.get(self.pos + 1) == Some(&'/') {
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

    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();
            if let Some('#') = self.peek_char() {
                while let Some(c) = self.peek_char() {
                    if c == '\n' {
                        break;
                    }
                    self.pos += 1;
                }
                continue;
            }
            if let (Some('/'), Some('*')) =
                (self.peek_char(), self.input.get(self.pos + 1).copied())
            {
                self.skip_multiline_comment();
                continue;
            }
            break;
        }

        match self.peek_char() {
            None => Token::Eof,
            Some(c) => {
                let next = self.input.get(self.pos + 1).copied();
                match (c, next) {
                    ('<', Some('=')) => {
                        self.pos += 2;
                        Token::LessEqual
                    }
                    ('>', Some('=')) => {
                        self.pos += 2;
                        Token::GreaterEqual
                    }
                    ('=', Some('=')) => {
                        self.pos += 2;
                        Token::Equal
                    }
                    ('!', Some('=')) => {
                        self.pos += 2;
                        Token::NotEqual
                    }
                    ('&', Some('&')) => {
                        self.pos += 2;
                        Token::And
                    }
                    ('|', Some('|')) => {
                        self.pos += 2;
                        Token::Or
                    }
                    _ => match c {
                        '+' => {
                            self.pos += 1;
                            Token::Plus
                        }
                        '-' => {
                            self.pos += 1;
                            Token::Minus
                        }
                        '*' => {
                            self.pos += 1;
                            Token::Star
                        }
                        '/' => {
                            self.pos += 1;
                            Token::Slash
                        }
                        '(' => {
                            self.pos += 1;
                            Token::LParen
                        }
                        ')' => {
                            self.pos += 1;
                            Token::RParen
                        }
                        '[' => {
                            self.pos += 1;
                            Token::LBracket
                        }
                        ']' => {
                            self.pos += 1;
                            Token::RBracket
                        }
                        '=' => {
                            self.pos += 1;
                            Token::Assign
                        }
                        ',' => {
                            self.pos += 1;
                            Token::Comma
                        }
                        ';' => {
                            self.pos += 1;
                            Token::Semicolon
                        }
                        '<' => {
                            self.pos += 1;
                            Token::Less
                        }
                        '>' => {
                            self.pos += 1;
                            Token::Greater
                        }
                        '!' => {
                            self.pos += 1;
                            Token::Not
                        }
                        '"' => {
                            let s = self.read_string();
                            Token::String(s)
                        }
                        _ if c.is_ascii_digit() || c == '.' => {
                            let num = self.read_number();
                            Token::Number(num)
                        }
                        _ if c.is_alphabetic() || c == '_' => {
                            let id = self.read_ident();
                            match id.as_str() {
                                "if" => Token::If,
                                "then" => Token::Then,
                                "else" => Token::Else,
                                "while" => Token::While,
                                "do" => Token::Do,
                                "fn" => Token::Fn,
                                _ => Token::Ident(id),
                            }
                        }
                        _ => panic!("Unexpected character: {}", c),
                    },
                }
            }
        }
    }
}
