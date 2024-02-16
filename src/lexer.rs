use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    i: usize,
    data: Vec<char>,

    current_column: usize,

    has_eof: bool,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let data = source.chars().into_iter().collect();

        Self {
            i: 0,
            data,
            current_column: 0,
            has_eof: false,
        }
    }

    fn advance(&mut self) {
        self.i += 1;
        self.current_column += 1;
    }

    fn current_as_char(&mut self) -> Option<char> {
        Some(*self.data.get(self.i)?)
    }

    fn skip_trash(&mut self) -> Option<()> {
        loop {
            let c = self.current_as_char()?;
            match c {
                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => return Some(()),
                _ => {
                    self.advance();
                    continue;
                }
            }
        }
    }

    fn collect_token(&mut self) -> Option<Token> {
        let c = self.current_as_char()?;
        self.advance();

        match c {
            '>' => Some(Token::Right),
            '<' => Some(Token::Left),
            '+' => Some(Token::Add),
            '-' => Some(Token::Subtract),
            '.' => Some(Token::PrintChar),
            ',' => Some(Token::GetChar),
            '[' => Some(Token::StartLoop),
            ']' => Some(Token::EndLoop),
            _ => None,
        }
    }

    fn eof(&mut self) -> Option<Token> {
        if self.has_eof {
            None
        } else {
            self.has_eof = true;
            Some(Token::EOF)
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.skip_trash() {
            return self.eof();
        }

        if let Some(value) = self.collect_token() {
            Some(value)
        } else {
            self.eof()
        }
    }
}
