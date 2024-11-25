use crate::base::OperatorKind;
use crate::tokenizer::{Token, Tokenizer};
use crate::tree::Node;

pub struct Parser<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new<'b>(tk: &'b mut Tokenizer<'b>) -> Parser<'b> {
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

    pub fn parse_expression(&mut self, inside_brace: bool) -> Box<Node> {
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
