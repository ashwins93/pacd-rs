use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PacdError {
    #[error("Cannot parse template file {0}")]
    CouldNotParseTemplate(String),
    #[error("Cannot render template for {0}")]
    CouldNotRenderFile(String),
    #[error("Cannot build parser")]
    CouldNotBuildParser,
    #[error("Collection key {0} cannot be found in data")]
    CollectionKeyNotFound(String),
    #[error("Data for key {0} is not a list. Expected {0} to be a list")]
    NoListAvailable(String),
    #[error("Collection items need to have an 'id' field: No ID found in {0}")]
    NoIDField(String),
    #[error("ID must be a string in {0} collection items")]
    IDParseError(String),
    #[error("Cannot convert bindings to object")]
    ObjectConversionError,
    #[error(
        "Cannot parse data file {0} into object. File does not exist or has incorrect data format."
    )]
    DataParseError(String),
    #[error("Error traversing directory")]
    TraverseError,
    #[error("Cannot create destination file/directory {0}. Check your permissions")]
    DestCreationError(String),
    #[error("Error reading contenst of the file {0}. Check your permissions")]
    SrcReadError(String),
    #[error("Something went wrong {0}")]
    PassThrough(Box<dyn std::error::Error>),
}
