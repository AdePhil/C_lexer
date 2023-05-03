use std::fs;

#[derive(Debug)]
struct Location {
    row: i32,
    col: i32,
    filename: String,
}
#[derive(Debug)]
enum TokenKind {
    PREPROCESSOR,
    Number,
    IDENTIFIER,
    OParen,
    CParen,
    OCurlyParen,
    CCurlyParen,
    INVALID,
    SemiColon,
    Comments,
}
#[derive(Debug)]
struct Token {
    value: String,
    kind: TokenKind,
}

struct Lexer<'a> {
    cursor: usize,
    start: usize,
    row: usize,
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        return Lexer {
            row: 0,
            cursor: 0,
            start: 0,
            content,
        };
    }

    fn chop_char(&mut self) {
        if self.is_not_empty() {
            let character = self.content[self.cursor];
            self.cursor += 1;
            if character == '\n' {
                self.row += 1;
                self.start = self.cursor;
            }
        }
    }
    fn trim_left(&mut self) {
        while self.is_not_empty() && self.content[self.cursor].is_whitespace() {
            self.chop_char();
        }
    }
    fn is_not_empty(&self) -> bool {
        return self.cursor < self.content.len();
    }
    fn next(&mut self) -> Option<Token> {
        self.trim_left();
        if !self.is_not_empty() {
            return None;
        }
        let first_char = self.content[self.cursor];
        // Process preprocessors
        if self.is_not_empty() && first_char == '#' {
            let index = self.cursor;
            while self.is_not_empty() && self.content[self.cursor] != '\n' {
                self.chop_char();
            }
            let value = self.content[index..self.cursor].iter().collect();
            if self.is_not_empty() {
                self.chop_char();
            }
            return Some(Token {
                value,
                kind: TokenKind::PREPROCESSOR,
            });
        }
        //Process IDENTIFIER
        if self.is_not_empty() && first_char == '-' || first_char.is_alphabetic() {
            let index = self.cursor;
            while self.is_not_empty() && self.content[self.cursor].is_alphanumeric() {
                self.chop_char();
            }
            return Some(Token {
                value: self.content[index..self.cursor].iter().collect(),
                kind: TokenKind::IDENTIFIER,
            });
        }
        //Process comments
        if self.is_not_empty()
            && self.content.len() > 2
            && self.content[self.cursor] == '/'
            && self.content[self.cursor + 1] == '/'
        {
            self.chop_char();
            self.chop_char();
            let index = self.cursor;
            while self.is_not_empty() && self.content[self.cursor] != '\n' {
                self.chop_char();
            }
            let value = self.content[index..self.cursor].iter().collect();
            if self.is_not_empty() {
                self.chop_char();
            }
            return Some(Token {
                value,
                kind: TokenKind::Comments,
            });
        }

        //Process digits
        if self.is_not_empty() && first_char.is_digit(10) {
            let index = self.cursor;
            while self.is_not_empty() && self.content[self.cursor].is_digit(10) {
                self.chop_char();
            }
            return Some(Token {
                value: self.content[index..self.cursor].iter().collect(),
                kind: TokenKind::Number,
            });
        }

        // TODO: use an hashmap
        let match_token = match first_char {
            '(' => Some(Token {
                value: self.content[self.cursor..self.cursor + 1].iter().collect(),
                kind: TokenKind::OParen,
            }),
            ')' => Some(Token {
                value: self.content[self.cursor..self.cursor + 1].iter().collect(),
                kind: TokenKind::CParen,
            }),
            '{' => Some(Token {
                value: self.content[self.cursor..self.cursor + 1].iter().collect(),
                kind: TokenKind::OCurlyParen,
            }),
            '}' => Some(Token {
                value: self.content[self.cursor..self.cursor + 1].iter().collect(),
                kind: TokenKind::CCurlyParen,
            }),
            ';' => Some(Token {
                value: self.content[self.cursor..self.cursor + 1].iter().collect(),
                kind: TokenKind::SemiColon,
            }),
            _ => None,
        };
        if match_token.is_some() {
            self.chop_char();
            return match_token;
        }

        if !self.is_not_empty() {
            return None;
        }

        let invalid_token = Some(Token {
            value: self.content[self.cursor..self.cursor + 1].iter().collect(),
            kind: TokenKind::INVALID,
        });
        self.chop_char();
        return invalid_token;
    }
}

fn main() {
    let file = "test.c";
    let content = fs::read_to_string(file).expect("Please give a valid path to file");

    let char_array: Vec<char> = content.chars().collect();
    let mut lexer = Lexer::new(&char_array);
    while let Some(token) = lexer.next() {
        print!("{} -> {:?}\n", token.value, token.kind);
    }
}
