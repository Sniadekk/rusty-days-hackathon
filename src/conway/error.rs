use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("index out of bounds")]
    IndexOutOfBounds(crate::conway::Index),
}
