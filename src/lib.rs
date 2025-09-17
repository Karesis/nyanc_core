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
