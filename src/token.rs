#[derive(PartialEq, Eq)]
pub enum TokenKind {
    Reserved,
    Num,
    Eof,
}

pub struct Token {
    pub kind: TokenKind,
    pub val: usize, // kindがTkNumの場合
    pub str: Vec<char>,
    pub pos: usize, // tokenizer.user_inputにおけるこのトークンの開始位置
}

impl Token {
    pub fn new(kind: TokenKind, val: usize, str: Vec<char>, pos: usize) -> Self {
        Token {
            kind,
            val,
            str,
            pos,
        }
    }
}
