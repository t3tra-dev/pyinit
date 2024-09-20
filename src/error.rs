#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to render template: {0}")]
    TemplateRendering(#[from] askama::Error),

    #[error("validation failed: {0}")]
    ArgValidation(&'static str),

    #[error("error in IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("failed to fetch: {0}")]
    Fetch(#[from] reqwest::Error),

    #[error("the library name '{name}' is already taken on PyPI")]
    LibraryNameTaken { name: String },
}
