use std::iter::Peekable;

use crate::{ast::AST, lexer::Lexer, token::Token};

pub struct Parser {
    lexer: Peekable<Lexer>,
    current: Token,
}

impl Parser {
    pub fn parse(mut lexer: Lexer) -> AST {
        let token = lexer.next().expect("Ran out of tokens");

        let mut parser = Parser {
            lexer: lexer.peekable(),
            current: token,
        };

        parser.parse_root()
    }

    fn advance(&mut self) -> Token {
        let current = self.current.clone();
        self.current = self.lexer.next().expect("Ran out of tokens");
        current
    }

    pub fn eat(&mut self, expected: Token) -> Token {
        if self.current == expected {
            let old = self.current.clone();
            self.advance();
            old
        } else {
            panic!("Missmatched tokens");
        }
    }

    pub fn parse_root(&mut self) -> AST {
        let mut statments = vec![];

        while self.current != Token::EOF {
            match self.current {
                Token::Add => {
                    statments.push(AST::Add(self.capture_number_of_occurances(Token::Add)));
                    continue;
                }
                Token::Subtract => {
                    statments.push(AST::Subtract(
                        self.capture_number_of_occurances(Token::Subtract),
                    ));
                    continue;
                }
                Token::Right => {
                    statments.push(AST::Right(self.capture_number_of_occurances(Token::Right)));
                    continue;
                }
                Token::Left => {
                    statments.push(AST::Left(self.capture_number_of_occurances(Token::Left)));
                    continue;
                }
                Token::GetChar => statments.push(AST::GetChar),
                Token::PrintChar => statments.push(AST::PrintChar),
                Token::StartLoop => {
                    statments.push(self.parse_loop());
                    continue;
                }
                Token::EndLoop => panic!("Unexpected end loop"),
                Token::EOF => break,
            }

            self.advance();
        }

        AST::Root(statments)
    }

    pub fn capture_number_of_occurances(&mut self, collecting: Token) -> usize {
        let mut amount = 0;

        while self.current == collecting {
            amount += 1;
            self.advance();
        }

        return amount;
    }

    pub fn parse_loop(&mut self) -> AST {
        self.eat(Token::StartLoop);
        let mut statments = vec![];

        while self.current != Token::EndLoop {
            match self.current {
                Token::Add => {
                    statments.push(AST::Add(self.capture_number_of_occurances(Token::Add)));
                    continue;
                }
                Token::Subtract => {
                    statments.push(AST::Subtract(
                        self.capture_number_of_occurances(Token::Subtract),
                    ));
                    continue;
                }
                Token::Right => {
                    statments.push(AST::Right(self.capture_number_of_occurances(Token::Right)));
                    continue;
                }
                Token::Left => {
                    statments.push(AST::Left(self.capture_number_of_occurances(Token::Left)));
                    continue;
                }
                Token::GetChar => statments.push(AST::GetChar),
                Token::PrintChar => statments.push(AST::PrintChar),
                Token::StartLoop => {
                    statments.push(self.parse_loop());
                    continue;
                }
                Token::EndLoop => panic!("Unreachable"),
                Token::EOF => panic!("Unexpected EOF"),
            }
            self.advance();
        }

        self.eat(Token::EndLoop);
        AST::Loop(statments)
    }
}
