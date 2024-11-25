use std::{
    fmt::{self},
    io::{self, Write},
    iter::Peekable,
    str::Chars,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum OperatorKind {
    ADD,
    SUB,
    MULT,
    DIV,
    POW
}

enum Node {
    Number(f32),
    Operation(OperatorKind, Box<Node>, Box<Node>),
}

fn evaluate(node: &Node) -> f32 {
    match &node {
        Node::Operation(operand, lnode, rnode) => {
            let lvalue = evaluate(lnode.as_ref());
            let rvalue = evaluate(rnode.as_ref());
            match operand {
                OperatorKind::ADD => lvalue + rvalue,
                OperatorKind::SUB => lvalue - rvalue,
                OperatorKind::MULT => lvalue * rvalue,
                OperatorKind::DIV => lvalue / rvalue,
                OperatorKind::POW => lvalue.powf(rvalue),
            }
        }
        Node::Number(val) => *val,
    }
}

fn draw_string(node: &Node, tab: String) -> String {
    match node {
        Node::Number(num) => format!("{}{}", tab, num.to_string()),
        Node::Operation(op, left, right) => format!(
            "{}{}\n{}\n{}",
            tab,
            match op {
                OperatorKind::ADD => "+",
                OperatorKind::SUB => "-",
                OperatorKind::MULT => "*",
                OperatorKind::DIV => "/",
                OperatorKind::POW => "^"
            },
            draw_string(&left, format!("{}{}", tab, "\t")),
            draw_string(&right, format!("{}{}", tab, "\t"))
        ),
    }
}

enum Token {
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

struct Tokenizer<'a> {
    line: Peekable<Chars<'a>>,
    token: Token,
}

impl<'a> Tokenizer<'a> {
    fn new<'b>(line: &'b String) -> Tokenizer<'b> {
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

    fn next_token(&mut self) {
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

struct Parser<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    fn new<'b>(tk: &'b mut Tokenizer<'b>) -> Parser<'b> {
        Parser { tokenizer: tk }
    }

    fn parse_term_base_item(&mut self) -> Box<Node> {
        match self.tokenizer.token {
            Token::BraceOpen => {
                self.tokenizer.next_token();
                self.parse_expression(true)
            }
            Token::Number(num) => {
                self.tokenizer.next_token();
                Box::new(Node::Number(num))
            }
            _ => panic!(
                "Expected number or brace open: Found {}",
                self.tokenizer.token
            ),
        }
    }

    fn parse_term_item(&mut self) -> Box<Node> {
        let mut left = self.parse_term_base_item();

        if let Token::Operator(op) = self.tokenizer.token {
            if op == OperatorKind::POW {
                self.tokenizer.next_token();

                left = Box::new(Node::Operation(op, left, self.parse_term_base_item()));
            }
        }

        left
    }

    fn parse_term(&mut self) -> Box<Node> {
        let mut left: Box<Node> = self.parse_term_item();

        loop {
            match self.tokenizer.token {
                Token::Operator(op) => match op {
                    OperatorKind::MULT | OperatorKind::DIV => {
                        self.tokenizer.next_token();
                        left = Box::new(Node::Operation(op, left, self.parse_term_item()))
                    }
                    _ => break,
                },
                Token::EOF => break,
                _ => break,
            };
        }

        left
    }

    fn parse_expression(&mut self, inside_brace: bool) -> Box<Node> {
        let mut left = self.parse_term();
        let mut closed = false;

        loop {
            match self.tokenizer.token {
                Token::Operator(op) => match op {
                    OperatorKind::ADD | OperatorKind::SUB => {
                        self.tokenizer.next_token();
                        left = Box::new(Node::Operation(op, left, self.parse_term()));
                    }
                    _ => panic!("Unexpected mult and div"),
                },
                Token::EOF => break,
                Token::BraceClose => {
                    if inside_brace {
                        closed = true;
                        self.tokenizer.next_token();
                        break;
                    } else {
                        panic!("Unexpected close bracket");
                    }
                }
                _ => panic!("Unexpecte token: {}", self.tokenizer.token),
            };
        }

        if inside_brace && !closed {
            panic!("Bracket not closed");
        }

        left
    }
}

fn main() {
    print!("Masukkan ekspresi : ");
    io::stdout().flush().expect("Flush gagal");

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Gagal membaca ekspresi");

    let mut tokenizer = Tokenizer::new(&line);
    tokenizer.next_token();

    let mut parser = Parser::new(&mut tokenizer);

    let node = parser.parse_expression(false);

    println!("{}", draw_string(&node, String::new()));

    println!("Evaluated to: {}", evaluate(&node));
}
