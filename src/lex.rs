#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    // <!
    CommentOpen,
    // </
    CloseTagOpen,
    // <
    TagOpen,
    // >
    TagClose,
    // abc
    Ident,
    // =
    Assign,
    // "aaa bbb"
    Attributes,
}

#[derive(Debug)]
pub struct Span {
    offset: u64,
    length: u64,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    span: Option<Span>,
}

impl Token {
    pub fn from_type(token_type: TokenType) -> Self {
        Self {
            token_type,
            span: None,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, rhs: &Self) -> bool {
        self.token_type == rhs.token_type
    }
}

#[derive(Default)]
pub struct Lexer {
    current: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn tokenize(&self, s: &str) -> Result<Vec<Token>, String> {
        Ok(vec![Token::from_type(TokenType::CommentOpen)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lex_comment() {
        let lexer = Lexer::new();
        assert_eq!(
            Ok(vec![Token::from_type(TokenType::CommentOpen)]),
            lexer.tokenize("<!"),
        );
    }
}
