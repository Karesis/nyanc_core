use super::Span;

#[derive(Debug, Clone)]
pub struct LexerError {
    // ... 具体的错误信息 ...
    pub span: Span,
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
