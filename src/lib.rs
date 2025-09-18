pub mod errors;
pub mod tokens;

pub type FileId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub file_id: FileId,
    pub start: usize,
    pub end: usize,
}

impl Span {
    // merge two spans into one
    // they should be in the same file
    pub fn merge(start: Self, end: Self) -> Self {
        assert_eq!(start.file_id, end.file_id);
        Self {
            file_id: start.file_id,
            start: start.start,
            end: end.end,
        }
    }
}

/// 代表一个被驻留的字符串的唯一ID。
/// 它轻量、可复制，并且可以用于高效的比较。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(pub u32);