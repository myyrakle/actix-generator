#[derive(Clone, Debug, PartialEq)]
pub struct TemplateError {
    pub message: String,
}

impl TemplateError {
    pub fn new<T: ToString>(message: T) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn boxed<T: ToString>(message: T) -> Box<Self> {
        Box::new(Self::new(message))
    }
}

impl std::error::Error for TemplateError {}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "parsing error: {}", self.message)
    }
}
