#[derive(thiserror::Error, Debug)]
pub enum Errors {
    #[error("ParseIntError({0})")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("ParseError")]
    ParseError,
    #[error("DimError({0})")]
    DimError(String),
    #[error("UncategorizedError({0})")]
    UncategorizedError(String),
}
