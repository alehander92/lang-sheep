use thiserror::Error;

#[derive(Error, Debug)]
#[error("error")]
pub enum LangSheepError {
    Io {
        #[from]
        source: std::io::Error,
        // backtrace: Backtrace,
    },
    ParseError,
    TypeError,
    GeneratorError
}
