use crate::base::OperatorKind;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

pub enum Token {
    Number(f32),
    Operator(OperatorKind),
    BraceOpen,
    BraceClose,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(_) => write!(f, "Number"),
            Token::Operator(_) => write!(f, "Operator"),
            Token::BraceOpen => write!(f, "BraceOpen"),
            Token::BraceClose => write!(f, "BraceClose"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

pub struct Tokenizer<'a> {
    line: Peekable<Chars<'a>>,
    pub token: Token,
}

impl<'a> Tokenizer<'a> {
    pub fn new<'b>(line: &'b String) -> Tokenizer<'b> {
        Tokenizer {
            line: line.chars().peekable(),
            token: Token::EOF,
        }
    }

    fn skip_white_space(&mut self) {
        while match self.line.peek() {
            None => false,
            Some(ch) => ch.is_whitespace(),
        } {
            self.line.next();
        }
    }

    fn read_number(&mut self) {
        let mut number = 0f32;

        loop {
            match self.line.peek() {
                None => break,
                Some(ch) => match ch.to_digit(10) {
                    None => break,
                    Some(digit) => {
                        let digitf: f32 = (digit as u8).into();
                        number = number * 10f32 + digitf;
                        self.line.next();
                    }
                },
            }
        }

        self.token = Token::Number(number)
    }

    pub fn next_token(&mut self) {
        self.skip_white_space();

        match self.line.peek() {
            None => self.token = Token::EOF,
            Some(ch) => {
                if ch.is_digit(10) {
                    self.read_number();
                    return;
                }

                self.token = match ch {
                    '+' => Token::Operator(OperatorKind::ADD),
                    '-' => Token::Operator(OperatorKind::SUB),
                    '*' => Token::Operator(OperatorKind::MULT),
                    '/' => Token::Operator(OperatorKind::DIV),
                    '^' => Token::Operator(OperatorKind::POW),
                    '(' => Token::BraceOpen,
                    ')' => Token::BraceClose,
                    _ => panic!("Unexpected token"),
                };
                self.line.next();
            }
        }
    }
}
