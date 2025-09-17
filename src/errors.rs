use super::Span;

#[derive(Debug, Clone)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub span: Span,
}

impl LexerError {
    pub fn new(kind: LexerErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone)]
pub enum LexerErrorKind {
    UnterminatedString,
    InvalidCharacter(char),
    InvalidEscapeSequence(char),
    UnnecessarySemicolon, 
    // 未来可以添加更多，比如数字格式错误等
}

#[derive(Debug, Clone)]
pub struct ParserError {
    // ... 具体的错误信息 ...
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum CompilerError {
    Lexer(LexerError),
    Parser(ParserError),
}

