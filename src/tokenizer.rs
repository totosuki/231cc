use crate::error::*;
use crate::token::*;

pub struct Tokenizer {
    tokens: Vec<Token>,
    pos: usize, // 現在何番目のトークンを見ているか
    user_input: String,
}

impl Tokenizer {
    pub fn new(tokens: Vec<Token>, pos: usize, user_input: String) -> Self {
        Tokenizer {
            tokens,
            pos,
            user_input,
        }
    }

    pub fn consume(&mut self, op: &[char]) -> bool {
        let token = &self.tokens[self.pos];
        if token.kind != TokenKind::Reserved || token.str != op {
            return false;
        }
        self.pos += 1;
        true
    }

    pub fn expect(&mut self, op: &[char]) {
        let token = &self.tokens[self.pos];
        if token.kind != TokenKind::Reserved || token.str != op {
            error(
                &self.user_input,
                &token.pos,
                &format!("{}ではありません", String::from_iter(op)),
            );
        }
        self.pos += 1;
    }

    pub fn expect_number(&mut self) -> usize {
        let token = &self.tokens[self.pos];
        if token.kind != TokenKind::Num {
            error(&self.user_input, &token.pos, "数ではありません");
        }
        let val = token.val;
        self.pos += 1;
        val
    }

    pub fn at_eof(&self) -> bool {
        let token = &self.tokens[self.pos];
        token.kind == TokenKind::Eof
    }

    pub fn new_token(
        &mut self,
        kind: TokenKind,
        val: Option<usize>, // EOFの場合Noneになる
        str: Vec<char>,
        pos: usize,
    ) {
        match val {
            Some(v) => self.tokens.push(Token::new(kind, v, str, pos)),
            None => self.tokens.push(Token::new(kind, 0, str, pos)),
        };
    }

    pub fn tokenize(&mut self, text: String) {
        let text: Vec<char> = text.chars().collect();
        let mut p: usize = 0;
        let mut t: char;

        while p < text.len() {
            t = text[p];
            if t.is_whitespace() {
                p += 1;
                continue;
            }

            if text[p..].starts_with(&['=', '='])
                || text[p..].starts_with(&['!', '='])
                || text[p..].starts_with(&['>', '='])
                || text[p..].starts_with(&['<', '='])
            {
                self.new_token(TokenKind::Reserved, None, vec![text[p], text[p + 1]], p);
                p += 2;
                continue;
            }

            if "+-*/()><".contains(t) {
                self.new_token(TokenKind::Reserved, None, vec![t], p);
                p += 1;
                continue;
            }

            let mut chars: Vec<char> = vec![];
            while t.is_digit(10) {
                chars.push(t);
                p += 1;
                if p >= text.len() {
                    break;
                }
                t = text[p];
            }
            if chars.len() > 0 {
                let numstr: String = chars.iter().collect();
                let num = Some(numstr.parse::<usize>().unwrap());
                self.new_token(TokenKind::Num, num, chars, p);
                continue;
            }

            error(&self.user_input, &p, "トークナイズできません。")
        }

        self.new_token(TokenKind::Eof, None, vec![], p);
    }
}
