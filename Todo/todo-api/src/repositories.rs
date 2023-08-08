pub mod todo;
pub mod label;

use thiserror::Error;

#[derive(Error, Debug)]
enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("NotFound, id id [{0}]")]
    NotFound(i32),
    #[error("Duplicate data, id is {0}")]
    Duplicate(i32),
}