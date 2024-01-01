#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    HTTP(#[from] reqwest::Error),
}
