/// An error validating the value of a custom string.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ValidationError {
    message: &'static str,
}

impl ValidationError {
    //! Construction

    /// Creates a new error with the given `message`.
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }

    /// Gets the error message.
    pub fn message(&self) -> &'static str {
        self.message
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}
