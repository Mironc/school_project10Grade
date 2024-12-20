#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOption{
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Always,
    Never,
    Equal
}
impl Into<u32> for CompareOption{
    fn into(self) -> u32 {
        match self {
            CompareOption::Less => gl::LESS,
            CompareOption::LessEqual => gl::LEQUAL,
            CompareOption::Greater => gl::GREATER,
            CompareOption::GreaterEqual => gl::GEQUAL,
            CompareOption::Always => gl::ALWAYS,
            CompareOption::Never => gl::NEVER,
            CompareOption::Equal => gl::EQUAL,
        }
    }
}