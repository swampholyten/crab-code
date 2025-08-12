use thiserror::Error;

#[derive(Error, Debug)]
pub enum JudgeError {
    #[error("Compilation error: {0}")]
    CompilationError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Time limit exceeded")]
    TimeLimitExceeded,

    #[error("Memory limit exceeded")]
    MemoryLimitExceeded,

    #[error("Wrong answer")]
    WrongAnswer,

    #[error("System error: {0}")]
    SystemError(String),

    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Code size limit exceeded")]
    CodeSizeLimitExceeded,

    #[error("Security violation: {0}")]
    SecurityViolation(String),
}
