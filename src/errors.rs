use crate::Span;
use crate::tokens::TokenType;

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
    pub kind: ParserErrorKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    /// 当遇到的 Token 不是期望的时。这是最常见的语法错误。
    /// e.g., "Expected a '}' to close the block, but found a 'fun' keyword instead."
    UnexpectedToken {
        expected: String, // 对期望内容的描述, e.g., "a type annotation" or "`}`"
        found: TokenType, // 实际遇到的 Token 类型
    },
    /// 当需要一个表达式，但没有找到时。
    /// e.g., "return ;" -> "Expected an expression after 'return'."
    ExpectedExpression,
    
    /// 当需要一个名称（标识符），但遇到了其他东西时。
    /// e.g., "fun 123() {}" -> "Expected a function name."
    ExpectedIdentifier,

    // 不允许顶层语句（即全局变量）
    NoToplevelStatements,
    // 我们可以根据需要添加更多...
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}


#[derive(Debug, Clone)]
pub enum CompilerError {
    Lexer(LexerError),
    Parser(ParserError),
}

